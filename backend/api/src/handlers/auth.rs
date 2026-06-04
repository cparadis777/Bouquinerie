use axum::extract::{Json as JsonExtractor, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum_login::AuthSession;
use chrono::Utc;
use db::state::AppState;
use domain::entities::users;
use password_auth::generate_hash;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::auth::{Backend, Credentials};
use crate::response::UserResponse;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Username or email already taken")
    )
)]
pub async fn register_handler(
    State(state): State<AppState>,
    mut auth: AuthSession<Backend>,
    JsonExtractor(body): JsonExtractor<RegisterRequest>,
) -> impl IntoResponse {
    let db = &state.db;

    let first_user = users::Entity::find()
        .order_by_asc(users::Column::Id)
        .one(db)
        .await
        .map_or(true, |u| u.is_none());

    let username_taken = users::Entity::find()
        .filter(users::Column::Username.eq(&body.username))
        .one(db)
        .await
        .ok()
        .flatten()
        .is_some();

    if username_taken {
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "username already taken"})),
        )
            .into_response();
    }

    let email_taken = users::Entity::find()
        .filter(users::Column::Email.eq(&body.email))
        .one(db)
        .await
        .ok()
        .flatten()
        .is_some();

    if email_taken {
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "email already taken"})),
        )
            .into_response();
    }

    let password_hash = generate_hash(body.password.as_bytes());

    let now = Utc::now().naive_utc();
    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(body.username),
        name: Set(body.name),
        email: Set(body.email),
        password_hash: Set(password_hash),
        is_admin: Set(first_user),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let inserted = match users::Entity::insert(user).exec(db).await {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to create user"})),
            )
                .into_response();
        }
    };

    let inserted_id = inserted.last_insert_id;
    match users::Entity::find_by_id(inserted_id).one(db).await {
        Ok(Some(user_model)) => {
            let user = crate::auth::User(user_model);
            let _ = auth.login(&user).await;
            let u = user.0;
            (
                StatusCode::CREATED,
                Json(UserResponse {
                    id: u.id,
                    username: u.username,
                    name: u.name,
                    email: u.email,
                    is_admin: u.is_admin,
                }),
            )
                .into_response()
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "user created but failed to retrieve"})),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = Credentials,
    responses(
        (status = 200, description = "Logged in", body = serde_json::Value),
        (status = 401, description = "Invalid credentials")
    )
)]
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

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Logged out", body = serde_json::Value)
    )
)]
pub async fn logout_handler(mut auth: AuthSession<Backend>) -> impl IntoResponse {
    let _ = auth.logout().await;
    (StatusCode::OK, Json(json!({"message": "logged out"}))).into_response()
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Current user", body = UserResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn me_handler(auth: AuthSession<Backend>) -> impl IntoResponse {
    match auth.user {
        Some(user) => {
            let u = user.0;
            (
                StatusCode::OK,
                Json(UserResponse {
                    id: u.id,
                    username: u.username,
                    name: u.name,
                    email: u.email,
                    is_admin: u.is_admin,
                }),
            )
                .into_response()
        }
        None => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "not authenticated"})),
        )
            .into_response(),
    }
}
