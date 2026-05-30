use axum::extract::{Path, Query, State};
use axum::Json;
use db::entities::{authors, books, identifiers, series};
use db::state::AppState;
use sea_orm::{EntityTrait, ModelTrait, PaginatorTrait, QueryOrder};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;
use crate::response::{BookListResponse, BookResponse, PaginationParams};

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
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).max(1).min(100);

    let paginator = books::Entity::find()
        .order_by_asc(books::Column::SortTitle)
        .paginate(&state.db, page_size);

    let items = paginator.fetch_page(page - 1).await?;
    let total = paginator.num_items().await?;
    let pages = paginator.num_pages().await?;

    Ok(Json(BookListResponse {
        data: items,
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
