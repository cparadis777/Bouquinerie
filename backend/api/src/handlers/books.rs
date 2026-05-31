use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::Json;
use db::entities::{authors, authors_books, books, identifiers, series, series_books};
use db::state::AppState;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;
use crate::response::{
    normalize_page, normalize_page_size, BookListEntry, BookListResponse, BookQueryParams,
    BookResponse,
};

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/books",
    params(BookQueryParams),
    responses(
        (status = 200, description = "List all books", body = BookListResponse)
    )
)]
pub async fn list_books(
    State(state): State<AppState>,
    Query(params): Query<BookQueryParams>,
) -> Result<Json<BookListResponse>, AppError> {
    let page = normalize_page(params.page);
    let page_size = normalize_page_size(params.page_size);

    let mut query = books::Entity::find().order_by_asc(books::Column::SortTitle);

    if let Some(author_id) = params.author_id {
        let book_ids: Vec<Uuid> = authors_books::Entity::find()
            .filter(authors_books::Column::AuthorId.eq(author_id))
            .all(&state.db)
            .await?
            .into_iter()
            .map(|ab| ab.book_id)
            .collect();
        query = query.filter(books::Column::Id.is_in(book_ids));
    }

    if let Some(series_id) = params.series_id {
        let book_ids: Vec<Uuid> = series_books::Entity::find()
            .filter(series_books::Column::SeriesId.eq(series_id))
            .all(&state.db)
            .await?
            .into_iter()
            .map(|sb| sb.book_id)
            .collect();
        query = query.filter(books::Column::Id.is_in(book_ids));
    }

    let paginator = query.paginate(&state.db, page_size);
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
