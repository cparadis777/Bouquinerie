use axum::extract::{Path, Query, State};
use axum::Json;
use domain::entities::series;
use db::state::AppState;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use tracing::instrument;
use uuid::Uuid;

use crate::error::AppError;
use crate::response::{normalize_page, normalize_page_size, PaginationParams, SeriesListResponse};

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/series",
    params(PaginationParams),
    responses(
        (status = 200, description = "List all series", body = SeriesListResponse)
    )
)]
pub async fn list_series(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<SeriesListResponse>, AppError> {
    let page = normalize_page(params.page);
    let page_size = normalize_page_size(params.page_size);

    let paginator = series::Entity::find()
        .order_by_asc(series::Column::Name)
        .paginate(&state.db, page_size);
    let items = paginator.fetch_page(page - 1).await?;
    let total = paginator.num_items().await?;
    let pages = paginator.num_pages().await?;

    Ok(Json(SeriesListResponse {
        data: items,
        total,
        page,
        pages,
        page_size,
    }))
}

#[instrument(skip(state), fields(series_id = %id))]
#[utoipa::path(
    get,
    path = "/api/series/{id}",
    responses(
        (status = 200, description = "Series by ID", body = domain::entities::series::Model),
        (status = 404, description = "Series not found")
    )
)]
pub async fn get_series(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<series::Model>, AppError> {
    let series = series::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Series not found".into()))?;

    Ok(Json(series))
}
