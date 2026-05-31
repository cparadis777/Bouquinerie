use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::Json;
use db::entities::{authors, authors_books, books, identifiers, series};
use db::state::AppState;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;
use crate::response::{
    normalize_page, normalize_page_size, BookListEntry, BookListResponse, BookResponse,
    PaginationParams,
};

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/books",
    params(PaginationParams),
    responses(
        (status = 200, description = "List all books", body = BookListResponse)
    )
)]
pub async fn list_books(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<BookListResponse>, AppError> {
    let page = normalize_page(params.page);
    let page_size = normalize_page_size(params.page_size);

    let paginator = books::Entity::find()
        .order_by_asc(books::Column::SortTitle)
        .paginate(&state.db, page_size);
    let items = paginator.fetch_page(page - 1).await?;
    let total = paginator.num_items().await?;
    let pages = paginator.num_pages().await?;

    let book_ids: Vec<Uuid> = items.iter().map(|b| b.id).collect();

    let author_map: HashMap<Uuid, Vec<String>> = if book_ids.is_empty() {
        HashMap::new()
    } else {
        authors_books::Entity::find()
            .filter(authors_books::Column::BookId.is_in(book_ids))
            .find_also_related(authors::Entity)
            .all(&state.db)
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut map, (ab, author_opt)| {
                if let Some(author) = author_opt {
                    map.entry(ab.book_id).or_default().push(author.name);
                }
                map
            })
    };

    let data: Vec<BookListEntry> = items
        .into_iter()
        .map(|book| {
            let author_names = author_map.get(&book.id).cloned().unwrap_or_default();
            BookListEntry { book, author_names }
        })
        .collect();

    Ok(Json(BookListResponse {
        data,
        total,
        page,
        pages,
        page_size,
    }))
}

#[instrument(skip(state), fields(book_id = %id))]
#[utoipa::path(
    get,
    path = "/api/books/{id}",
    responses(
        (status = 200, description = "Book with relations", body = BookResponse),
        (status = 404, description = "Book not found")
    )
)]
pub async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookResponse>, AppError> {
    let book = books::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Book not found".into()))?;

    let authors = book.find_related(authors::Entity).all(&state.db).await?;
    let series = book.find_related(series::Entity).all(&state.db).await?;
    let identifiers = book.find_related(identifiers::Entity).all(&state.db).await?;

    Ok(Json(BookResponse {
        book,
        authors,
        series,
        identifiers,
    }))
}
