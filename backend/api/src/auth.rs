use axum_login::{AuthUser, AuthnBackend, UserId};
use domain::entities::users;
use password_auth::verify_password;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User(pub users::Model);

#[derive(Clone, Debug)]
pub struct Backend {
    pub db: DatabaseConnection,
}

#[derive(Clone, Debug, Deserialize, utoipa::ToSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("database error: {0}")]
    Db(#[from] sea_orm::DbErr),
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.0.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.0.password_hash.as_bytes()
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(&creds.username))
            .one(&self.db)
            .await?;

        match user {
            Some(u) if verify_password(&creds.password, &u.password_hash).is_ok() => {
                Ok(Some(User(u)))
            }
            _ => Ok(None),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(users::Entity::find_by_id(*user_id)
            .one(&self.db)
            .await?
            .map(User))
    }
}
