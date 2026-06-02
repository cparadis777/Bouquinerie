use async_trait::async_trait;
use uuid::Uuid;

use domain::entities::series::Model as Series;

use crate::error::RepositoryError;

mod db;
#[cfg(feature = "mock")]
mod mock;

pub use db::DbSeriesRepository;
#[cfg(feature = "mock")]
pub use mock::MockSeriesRepository;

#[async_trait]
pub trait SeriesRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Series>, RepositoryError>;
    async fn list(&self, params: SeriesListParams) -> Result<(Vec<Series>, u64), RepositoryError>;
}

pub struct SeriesListParams {
    pub page: u64,
    pub page_size: u64,
}
