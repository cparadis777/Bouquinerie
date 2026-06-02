#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[cfg(feature = "mock")]
    #[error("Mock error: {0}")]
    Mock(&'static str),
}
