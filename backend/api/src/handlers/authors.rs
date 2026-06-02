use axum::Json;
use axum::extract::{Path, Query, State};
use db::state::AppState;
use domain::entities::authors;
use repository::authors::{AuthorListParams, AuthorRepository, DbAuthorRepository};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;
use crate::response::{AuthorListResponse, PaginationParams, normalize_page, normalize_page_size};

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/authors",
    params(PaginationParams),
    responses(
        (status = 200, description = "List all authors", body = AuthorListResponse)
    )
)]
pub async fn list_authors(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<AuthorListResponse>, AppError> {
    let page = normalize_page(params.page);
    let page_size = normalize_page_size(params.page_size);
    let repo = DbAuthorRepository::new(&state.db);
    let (items, total) = repo.list(AuthorListParams { page, page_size }).await?;
    let pages = total.div_ceil(page_size);

    Ok(Json(AuthorListResponse {
        data: items,
        total,
        page,
        pages,
        page_size,
    }))
}

#[instrument(skip(state), fields(author_id = %id))]
#[utoipa::path(
    get,
    path = "/api/authors/{id}",
    responses(
        (status = 200, description = "Author by ID", body = domain::entities::authors::Model),
        (status = 404, description = "Author not found")
    )
)]
pub async fn get_author(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<authors::Model>, AppError> {
    let repo = DbAuthorRepository::new(&state.db);
    let author = repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Author not found".into()))?;
    Ok(Json(author))
}
