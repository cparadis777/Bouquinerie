use axum::Json;
use axum::extract::{Path, Query, State};
use db::state::AppState;
use domain::entities::series;
use tracing::instrument;
use uuid::Uuid;

use repository::series::{DbSeriesRepository, SeriesListParams, SeriesRepository};

use crate::error::AppError;
use crate::response::{PaginationParams, SeriesListResponse, normalize_page, normalize_page_size};

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

    let repo = DbSeriesRepository::new(&state.db);
    let (items, total) = repo.list(SeriesListParams { page, page_size }).await?;
    let pages = total.div_ceil(page_size);

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
    let repo = DbSeriesRepository::new(&state.db);

    let series = repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Series not found".into()))?;

    Ok(Json(series))
}
