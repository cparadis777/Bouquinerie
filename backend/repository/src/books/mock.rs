use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

use domain::entities::{
    authors::Model as Author, books::Model as Book, identifiers::Model as Identifier,
    series::Model as Series,
};

use super::{BookListParams, BookListResult, BookRepository};
use crate::error::RepositoryError;

pub struct MockBookRepository {
    pub books: Vec<Book>,
    pub authors: Vec<Author>,
    pub series: Vec<Series>,
    pub identifiers: Vec<Identifier>,
    pub should_fail: bool,
}

impl MockBookRepository {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            authors: Vec::new(),
            series: Vec::new(),
            identifiers: Vec::new(),
            should_fail: false,
        }
    }
}

#[async_trait]
impl BookRepository for MockBookRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Book>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.books.iter().find(|b| b.id == id).cloned())
    }

    async fn list(&self, params: BookListParams) -> Result<BookListResult, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        let total = self.books.len() as u64;
        let start = ((params.page - 1) * params.page_size) as usize;
        let items = self
            .books
            .iter()
            .skip(start)
            .take(params.page_size as usize)
            .cloned()
            .collect();

        Ok(BookListResult {
            items,
            author_names: HashMap::new(),
            total,
        })
    }

    async fn find_authors(&self, _book: &Book) -> Result<Vec<Author>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.authors.clone())
    }

    async fn find_series(&self, _book: &Book) -> Result<Vec<Series>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.series.clone())
    }

    async fn find_identifiers(&self, _book: &Book) -> Result<Vec<Identifier>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.identifiers.clone())
    }
}
