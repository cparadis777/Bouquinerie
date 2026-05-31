use db::entities::books;
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
        Number(u64),
        Text(String),
    }

    match OneOf::deserialize(deserializer)? {
        OneOf::Number(v) => Ok(Some(v)),
        OneOf::Text(s) if s.is_empty() || s == "null" => Ok(None),
        OneOf::Text(s) => s.parse::<u64>().map(Some).map_err(de::Error::custom),
    }
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct PaginationParams {
    #[serde(default, deserialize_with = "deserialize_nullable_u64")]
    pub page: Option<u64>,
    #[serde(default, deserialize_with = "deserialize_nullable_u64")]
    pub page_size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct BookListEntry {
    pub book: books::Model,
    pub author_names: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BookListResponse {
    pub data: Vec<BookListEntry>,
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
