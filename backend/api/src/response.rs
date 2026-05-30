use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub pages: u64,
    pub page_size: u64,
}

#[derive(Serialize, ToSchema)]
pub struct BookResponse {
    pub book: db::entities::books::Model,
    pub authors: Vec<db::entities::authors::Model>,
    pub series: Vec<db::entities::series::Model>,
    pub identifiers: Vec<db::entities::identifiers::Model>,
}
