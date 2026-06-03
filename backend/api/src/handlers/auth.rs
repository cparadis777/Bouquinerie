use axum::extract::Json as JsonExtractor;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum_login::AuthSession;
use serde_json::json;

use crate::auth::{Backend, Credentials};

pub async fn login_handler(
    mut auth: AuthSession<Backend>,
    JsonExtractor(creds): JsonExtractor<Credentials>,
) -> impl IntoResponse {
    let user = match auth.authenticate(creds).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "invalid credentials"})),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "authentication failed"})),
            )
                .into_response();
        }
    };

    if auth.login(&user).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "login failed"})),
        )
            .into_response();
    }

    (StatusCode::OK, Json(json!({"message": "logged in"}))).into_response()
}

pub async fn logout_handler(mut auth: AuthSession<Backend>) -> impl IntoResponse {
    let _ = auth.logout().await;
    (StatusCode::OK, Json(json!({"message": "logged out"}))).into_response()
}
