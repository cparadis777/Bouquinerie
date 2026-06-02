use async_trait::async_trait;
use uuid::Uuid;

use domain::entities::authors::Model as Author;

use crate::error::RepositoryError;

mod db;
#[cfg(feature = "mock")]
mod mock;

pub use db::DbAuthorRepository;
#[cfg(feature = "mock")]
pub use mock::MockAuthorRepository;

#[async_trait]
pub trait AuthorRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Author>, RepositoryError>;
    async fn list(&self, params: AuthorListParams) -> Result<(Vec<Author>, u64), RepositoryError>;
}

pub struct AuthorListParams {
    pub page: u64,
    pub page_size: u64,
}
