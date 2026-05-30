use axum::extract::{Query, State};
use axum::Json;
use db::entities::authors;
use db::state::AppState;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tracing::instrument;

use crate::error::AppError;
use crate::response::{AuthorListResponse, PaginationParams};

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
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).max(1).min(100);

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
