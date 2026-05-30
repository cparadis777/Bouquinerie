use axum::extract::{Path, State};
use axum::Json;
use db::entities::{authors, books, identifiers, series};
use db::state::AppState;
use sea_orm::{EntityTrait, ModelTrait, QueryOrder};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/books",
    responses(
        (status = 200, description = "List all books", body = Vec<books::Model>)
    )
)]
pub async fn list_books(
    State(state): State<AppState>,
) -> Result<Json<Vec<books::Model>>, AppError> {
    let books = books::Entity::find()
        .order_by_asc(books::Column::SortTitle)
        .all(&state.db)
        .await?;
    Ok(Json(books))
}

#[instrument(skip(state), fields(book_id = %id))]
#[utoipa::path(
    get,
    path = "/api/books/{id}",
    responses(
        (status = 200, description = "Book with relations"),
        (status = 404, description = "Book not found")
    )
)]
pub async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let book = books::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Book not found".into()))?;

    let authors = book.find_related(authors::Entity).all(&state.db).await?;
    let series = book.find_related(series::Entity).all(&state.db).await?;
    let identifiers = book.find_related(identifiers::Entity).all(&state.db).await?;

    Ok(Json(serde_json::json!({
        "book": book,
        "authors": authors,
        "series": series,
        "identifiers": identifiers,
    })))
}
