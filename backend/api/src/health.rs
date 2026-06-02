use axum::Json;
use axum::extract::State;
use db::state::AppState;
use tracing::instrument;

use crate::error::AppError;

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 503, description = "Service unhealthy")
    )
)]
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    state.db.ping().await?;
    Ok(Json(
        serde_json::json!({ "status": "ok", "database": "connected" }),
    ))
}
