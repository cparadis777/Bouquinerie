use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use repository::error::RepositoryError;

#[derive(Debug, thiserror::Error)]
#[expect(dead_code)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Repository(#[from] repository::error::RepositoryError),

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
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::Repository(err) => match &err {
                RepositoryError::NotFound(msg) => {
                    tracing::warn!(?err, "resource not found");
                    (StatusCode::NOT_FOUND, msg.clone())
                }
                RepositoryError::Database(dbe) => {
                    tracing::error!(?dbe, "database error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "A database error occured".to_string(),
                    )
                }
                #[cfg(feature = "mock")]
                RepositoryError::Mock(msg) => {
                    tracing::error!(?msg, "mock error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "An internal error occured".to_string(),
                    )
                }
            },
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
