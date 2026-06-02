use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Database(sea_orm::DbErr),
    #[cfg(feature = "mock")]
    Mock(&'static str),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::Database(err) => write!(f, "Database error: {err}"),
            #[cfg(feature = "mock")]
            Self::Mock(msg) => write!(f, "Mock error: {msg}"),
        }
    }
}

impl std::error::Error for RepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(err) => Some(err),
            _ => None,
        }
    }
}

impl From<sea_orm::DbErr> for RepositoryError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Database(err)
    }
}
