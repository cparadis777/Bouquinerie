use std::path::{Path, PathBuf};

use db::entities::{authors, authors_books, book_files, books};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::config::Config;
use crate::library;
use crate::metadata::{self, BookMeta};

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("metadata extraction failed: {0}")]
    Metadata(#[from] metadata::ExtractError),
    #[error("library operation failed: {0}")]
    Library(#[from] library::LibraryError),
    #[error("database operation failed: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("no authors provided")]
    NoAuthors,
    #[error("duplicate file (hash already exists): {0}")]
    Duplicate(String),
}

pub struct IngestResult {
    pub book_id: Uuid,
    pub book_path: PathBuf,
    pub book_dir: PathBuf,
}

#[instrument(skip(db, config), fields(file = %file_path.display()))]
pub async fn ingest(
    db: &sea_orm::DatabaseConnection,
    config: &Config,
    file_path: &Path,
) -> Result<IngestResult, ProcessError> {
    // 1. Extract metadata
    let meta = metadata::extract_metadata(file_path)?;

    // 2. Compute file properties
    let file_hash = library::hash_file(file_path).await?;
    let size = library::file_size(file_path).await?;

    // 3. Check for duplicate hash
    let existing = book_files::Entity::find()
        .filter(book_files::Column::FileHash.eq(&file_hash))
        .one(db)
        .await?;
    if existing.is_some() {
        return Err(ProcessError::Duplicate(file_hash));
    }

    // 4. Build library destination (deterministic from metadata)
    let rel_path = library::build_rel_path(&meta);
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| {
            library::LibraryError::PathValidation("invalid filename".into())
        })?;
    let rel_path_str = rel_path.to_string_lossy().to_string();

    // 5. Begin database transaction — file stays in /incoming until commit succeeds
    let txn = db.begin().await?;

    // 6. Upsert authors
    let author_ids = upsert_authors(&txn, &meta).await?;

    // 7. Create book record
    let book_id = Uuid::new_v4();
    let now = chrono::Utc::now().naive_utc();
    let published_date = meta.published_date
        .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

    let book = books::ActiveModel {
        id: Set(book_id),
        title: Set(meta.title),
        subtitle: Set(meta.subtitle.unwrap_or_default()),
        description: Set(meta.description.unwrap_or_default().into()),
        language: Set(meta.language),
        publisher: Set(meta.publisher.unwrap_or_default()),
        isbn: Set(meta.isbn.unwrap_or_default()),
        page_count: Set(meta.page_count.unwrap_or(0)),
        cover_path: Set(String::new()),
        published_date: Set(published_date),
        created_at: Set(now),
        updated_at: Set(now),
        sort_title: NotSet,
        book_path: Set(Some(rel_path_str)),
        file_hash: Set(Some(file_hash.clone())),
        size_bytes: Set(Some(size)),
    };
    book.insert(&txn).await?;

    // 8. Create authors_books join records
    for (i, author_id) in author_ids.iter().enumerate() {
        let join = authors_books::ActiveModel {
            book_id: Set(book_id),
            author_id: Set(*author_id),
            sort_order: Set(i as i32),
            is_primary: Set(i == 0),
        };
        join.insert(&txn).await?;
    }

    // 9. Create book_files record
    let bf = book_files::ActiveModel {
        id: Set(Uuid::new_v4()),
        book_id: Set(book_id),
        medium: Set(meta.medium.as_str().to_string()),
        file_path: Set(filename.to_string()),
        format: Set(Some(meta.format)),
        file_hash: Set(Some(file_hash)),
        size_bytes: Set(Some(size)),
    };
    bf.insert(&txn).await?;

    // 10. Commit — all-or-nothing. File still in /incoming if this fails.
    txn.commit().await?;

    // --- All DB writes committed, safe to move the file ---

    // 11. Move file to library
    let (dest_path, _) = library::move_to_library(
        file_path,
        &config.library_path,
        &rel_path,
        filename,
    )
    .await?;

    let book_dir = dest_path.parent().unwrap_or(&config.library_path).to_path_buf();

    // 12. Save cover if available (non-fatal)
    if let Some(cover_data) = meta.cover_data {
        match library::save_cover(&book_dir, &cover_data).await {
            Ok(cover_path) => {
                let cover_str = cover_path.to_string_lossy().to_string();
                let _ = books::Entity::update_many()
                    .col_expr(books::Column::CoverPath, cover_str.into())
                    .filter(books::Column::Id.eq(book_id))
                    .exec(db)
                    .await;
            }
            Err(e) => warn!(error = %e, "failed to save cover, skipping"),
        }
    }

    Ok(IngestResult {
        book_id,
        book_path: dest_path,
        book_dir,
    })
}

async fn upsert_author<C: ConnectionTrait>(
    db: &C,
    name: &str,
) -> Result<(Uuid, bool), sea_orm::DbErr> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err(sea_orm::DbErr::Custom("empty author name".into()));
    }

    let existing = authors::Entity::find()
        .filter(authors::Column::Name.eq(&trimmed))
        .one(db)
        .await?;

    if let Some(author) = existing {
        return Ok((author.id, false));
    }

    let new = authors::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(trimmed.clone()),
        sort_name: Set(library::sort_name(&trimmed)),
    };
    let result = new.insert(db).await?;
    Ok((result.id, true))
}

async fn upsert_authors<C: ConnectionTrait>(
    db: &C,
    meta: &BookMeta,
) -> Result<Vec<Uuid>, ProcessError> {
    if meta.authors.is_empty() {
        return Err(ProcessError::NoAuthors);
    }

    let mut ids = Vec::new();
    for name in &meta.authors {
        let (id, _created) = upsert_author(db, name).await?;
        ids.push(id);
    }
    Ok(ids)
}
