use async_trait::async_trait;
use uuid::Uuid;

use domain::entities::series::Model as Series;

use super::{SeriesListParams, SeriesRepository};
use crate::error::RepositoryError;

pub struct MockSeriesRepository {
    pub series: Vec<Series>,
    pub should_fail: bool,
}

impl MockSeriesRepository {
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            should_fail: false,
        }
    }
}

#[async_trait]
impl SeriesRepository for MockSeriesRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Series>, RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        Ok(self.series.iter().find(|s| s.id == id).cloned())
    }

    async fn list(&self, params: SeriesListParams) -> Result<(Vec<Series>, u64), RepositoryError> {
        if self.should_fail {
            return Err(RepositoryError::Mock("forced failure"));
        }
        let total = self.series.len() as u64;
        let start = ((params.page - 1) * params.page_size) as usize;
        let items = self
            .series
            .iter()
            .skip(start)
            .take(params.page_size as usize)
            .cloned()
            .collect();
        Ok((items, total))
    }
}
