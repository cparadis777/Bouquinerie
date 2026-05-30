use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use db::state::AppState;

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
) -> (StatusCode, Json<serde_json::Value>) {
    match state.db.ping().await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "status": "ok", "database": "connected" })),
        ),
        Err(e) => {
            tracing::warn!("Health check — database ping failed: {e}");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "status": "error", "database": "disconnected" })),
            )
        }
    }
}
