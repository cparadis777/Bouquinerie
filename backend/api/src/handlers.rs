use axum::Json;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check passed", body = serde_json::Value)
    )
)]
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}
