use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use domain::entities::{
    authors::Model as Author, books::Model as Book, identifiers::Model as Identifier,
    series::Model as Series,
};

use crate::error::RepositoryError;

mod db;
#[cfg(feature = "mock")]
mod mock;

pub use db::DbBookRepository;
#[cfg(feature = "mock")]
pub use mock::MockBookRepository;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Book>, RepositoryError>;
    async fn list(&self, params: BookListParams) -> Result<BookListResult, RepositoryError>;
    async fn find_authors(&self, book: &Book) -> Result<Vec<Author>, RepositoryError>;
    async fn find_series(&self, book: &Book) -> Result<Vec<Series>, RepositoryError>;
    async fn find_identifiers(&self, book: &Book) -> Result<Vec<Identifier>, RepositoryError>;
}

pub struct BookListParams {
    pub page: u64,
    pub page_size: u64,
    pub author_id: Option<Uuid>,
    pub series_id: Option<Uuid>,
}

pub struct BookListResult {
    pub items: Vec<Book>,
    pub author_names: HashMap<Uuid, Vec<String>>,
    pub total: u64,
}
