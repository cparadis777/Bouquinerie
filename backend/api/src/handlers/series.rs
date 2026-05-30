use axum::extract::State;
use axum::Json;
use db::entities::series;
use db::state::AppState;
use sea_orm::{EntityTrait, QueryOrder};
use tracing::instrument;

use crate::error::AppError;

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/api/series",
    responses(
        (status = 200, description = "List all series", body = Vec<series::Model>)
    )
)]
pub async fn list_series(
    State(state): State<AppState>,
) -> Result<Json<Vec<series::Model>>, AppError> {
    let series = series::Entity::find()
        .order_by_asc(series::Column::Name)
        .all(&state.db)
        .await?;
    Ok(Json(series))
}
