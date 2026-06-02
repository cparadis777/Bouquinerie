use async_trait::async_trait;
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder};
use uuid::Uuid;

use domain::entities::series;

use super::{SeriesListParams, SeriesRepository};
use crate::error::RepositoryError;

pub struct DbSeriesRepository<'a, C: ConnectionTrait> {
    db: &'a C,
}

impl<'a, C: ConnectionTrait> DbSeriesRepository<'a, C> {
    pub fn new(db: &'a C) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a, C: ConnectionTrait + Send + Sync> SeriesRepository for DbSeriesRepository<'a, C> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<series::Model>, RepositoryError> {
        Ok(series::Entity::find_by_id(id).one(self.db).await?)
    }

    async fn list(
        &self,
        params: SeriesListParams,
    ) -> Result<(Vec<series::Model>, u64), RepositoryError> {
        let paginator = series::Entity::find()
            .order_by_asc(series::Column::Name)
            .paginate(self.db, params.page_size);
        let items = paginator.fetch_page(params.page - 1).await?;
        let total = paginator.num_items().await?;
        Ok((items, total))
    }
}
