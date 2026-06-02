use crate::error::AppError;
use crate::response::{
    BookListEntry, BookListResponse, BookQueryParams, BookResponse, normalize_page,
    normalize_page_size,
};
use axum::Json;
use axum::extract::{Path, Query, State};
use db::state::AppState;
use repository::books::{BookListParams, BookRepository, DbBookRepository};
use tracing::instrument;
use uuid::Uuid;

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
    let repo = DbBookRepository::new(&state.db);

    let mut result = repo
        .list(BookListParams {
            page,
            page_size,
            author_id: params.author_id,
            series_id: params.series_id,
        })
        .await?;

    let total = result.total;
    let items = result.items;
    let pages = total.div_ceil(page_size);

    let data: Vec<BookListEntry> = items
        .into_iter()
        .map(|book| {
            let author_names = result.author_names.remove(&book.id).unwrap_or_default();
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
    let repo = DbBookRepository::new(&state.db);
    let book = repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Book not found".into()))?;

    let authors = repo.find_authors(&book).await?;
    let series = repo.find_series(&book).await?;
    let identifiers = repo.find_identifiers(&book).await?;

    Ok(Json(BookResponse {
        book,
        authors,
        series,
        identifiers,
    }))
}
