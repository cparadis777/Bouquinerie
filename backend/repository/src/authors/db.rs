use async_trait::async_trait;
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder};
use uuid::Uuid;

use domain::entities::authors;

use super::{AuthorListParams, AuthorRepository};
use crate::error::RepositoryError;

pub struct DbAuthorRepository<'a, C: ConnectionTrait> {
    db: &'a C,
}

impl<'a, C: ConnectionTrait> DbAuthorRepository<'a, C> {
    pub fn new(db: &'a C) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a, C: ConnectionTrait + Send + Sync> AuthorRepository for DbAuthorRepository<'a, C> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<authors::Model>, RepositoryError> {
        Ok(authors::Entity::find_by_id(id).one(self.db).await?)
    }

    async fn list(
        &self,
        params: AuthorListParams,
    ) -> Result<(Vec<authors::Model>, u64), RepositoryError> {
        let paginator = authors::Entity::find()
            .order_by_asc(authors::Column::SortName)
            .paginate(self.db, params.page_size);
        let items = paginator.fetch_page(params.page - 1).await?;
        let total = paginator.num_items().await?;
        Ok((items, total))
    }
}
