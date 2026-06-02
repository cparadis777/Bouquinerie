use async_trait::async_trait;
use uuid::Uuid;

use domain::entities::authors::Model as Author;

use super::{AuthorListParams, AuthorRepository};
use crate::error::RepositoryError;

pub struct MockAuthorRepository {
    pub authors: Vec<Author>,
    pub should_fail: bool,
}

impl MockAuthorRepository {
    pub fn new() -> Self {
        Self {
            authors: Vec::new(),
            should_fail: false,
        }
    }
}

#[async_trait]
impl AuthorRepository for MockAuthorRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Author>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.authors.iter().find(|a| a.id == id).cloned())
    }

    async fn list(&self, params: AuthorListParams) -> Result<(Vec<Author>, u64), RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        let total = self.authors.len() as u64;
        let start = ((params.page - 1) * params.page_size) as usize;
        let items = self
            .authors
            .iter()
            .skip(start)
            .take(params.page_size as usize)
            .cloned()
            .collect();
        Ok((items, total))
    }
}
