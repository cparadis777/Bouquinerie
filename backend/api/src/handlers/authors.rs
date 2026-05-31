use axum::extract::{Query, State};
use axum::Json;
use db::entities::authors;
use db::state::AppState;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tracing::instrument;

use crate::error::AppError;
use crate::response::{
    normalize_page, normalize_page_size, AuthorListResponse, PaginationParams,
};

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

    let paginator = authors::Entity::find()
        .order_by_asc(authors::Column::SortName)
        .paginate(&state.db, page_size);
    let items = paginator.fetch_page(page - 1).await?;
    let total = paginator.num_items().await?;
    let pages = paginator.num_pages().await?;

    Ok(Json(AuthorListResponse {
        data: items,
        total,
        page,
        pages,
        page_size,
    }))
}
