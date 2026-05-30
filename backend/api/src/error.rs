use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Database(_) => {
                tracing::error!(?self, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!(?self, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg.clone(),
                )
            }
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
