use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

fn deserialize_nullable_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOf {
        U64(u64),
        Null(String),
    }

    match OneOf::deserialize(deserializer)? {
        OneOf::U64(v) => Ok(Some(v)),
        OneOf::Null(s) if s == "null" || s.is_empty() => Ok(None),
        OneOf::Null(s) => Err(de::Error::custom(format!("invalid value: {s}"))),
    }
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct PaginationParams {
    #[serde(deserialize_with = "deserialize_nullable_u64")]
    pub page: Option<u64>,
    #[serde(deserialize_with = "deserialize_nullable_u64")]
    pub page_size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct BookListResponse {
    pub data: Vec<db::entities::books::Model>,
    pub total: u64,
    pub page: u64,
    pub pages: u64,
    pub page_size: u64,
}

#[derive(Serialize, ToSchema)]
pub struct AuthorListResponse {
    pub data: Vec<db::entities::authors::Model>,
    pub total: u64,
    pub page: u64,
    pub pages: u64,
    pub page_size: u64,
}

#[derive(Serialize, ToSchema)]
pub struct SeriesListResponse {
    pub data: Vec<db::entities::series::Model>,
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

pub fn normalize_page(page: Option<u64>) -> u64 {
    page.unwrap_or(1).max(1)
}

pub fn normalize_page_size(size: Option<u64>) -> u64 {
    size.unwrap_or(20).max(1).min(100)
}
