use std::path::Path;

use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Medium {
    Ebook,
    Audiobook,
    Unknown,
}

impl Medium {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ebook => "ebook",
            Self::Audiobook => "audiobook",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BookMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Vec<String>,
    pub isbn: Option<String>,
    pub publisher: Option<String>,
    pub language: String,
    pub published_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub page_count: Option<i32>,
    pub cover_data: Option<Vec<u8>>,
    pub medium: Medium,
    pub format: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("failed to open file: {0}")]
    OpenFailed(String),
    #[error("missing required metadata: {0}")]
    MissingMetadata(&'static str),
}

pub fn extract_metadata(path: &Path) -> Result<BookMeta, ExtractError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .ok_or_else(|| ExtractError::UnsupportedFormat("no extension".into()))?;

    match ext.as_str() {
        "epub" => self::epub::extract(path),
        _ => Err(ExtractError::UnsupportedFormat(ext)),
    }
}

fn detect_medium(ext: &str) -> Medium {
    match ext {
        "epub" | "pdf" | "mobi" | "azw3" => Medium::Ebook,
        "m4b" | "mp3" | "aax" | "flac" | "ogg" => Medium::Audiobook,
        _ => Medium::Unknown,
    }
}

mod epub {
    use std::path::Path;

    use chrono::NaiveDate;

    use super::{BookMeta, ExtractError, detect_medium};

    pub fn extract(path: &Path) -> Result<BookMeta, ExtractError> {
        let mut doc =
            epub::doc::EpubDoc::new(path).map_err(|e| ExtractError::OpenFailed(e.to_string()))?;

        let title = doc
            .mdata("title")
            .map(|m| m.value.clone())
            .filter(|v| !v.is_empty())
            .ok_or(ExtractError::MissingMetadata("title"))?;

        let mut authors: Vec<String> = Vec::new();
        for item in &doc.metadata {
            if item.property == "creator" {
                let name = item.value.trim().to_string();
                if !name.is_empty() && !authors.contains(&name) {
                    authors.push(name);
                }
            }
        }
        if authors.is_empty() {
            authors.push("Unknown Author".to_string());
        }

        let isbn = doc.mdata("identifier").map(|m| {
            let v = m.value.trim().to_string();
            if v.starts_with("urn:isbn:") {
                v.trim_start_matches("urn:isbn:").to_string()
            } else {
                v
            }
        });

        let publisher = doc.mdata("publisher").map(|m| m.value.clone());

        let language = doc
            .mdata("language")
            .map(|m| m.value.clone())
            .unwrap_or_else(|| "en".to_string());

        let published_date = doc.mdata("date").and_then(|m| {
            let v = m.value.trim().to_string();
            NaiveDate::parse_from_str(&v, "%Y-%m-%d")
                .ok()
                .or_else(|| NaiveDate::parse_from_str(&v, "%Y").ok())
        });

        let description = doc.mdata("description").map(|m| m.value.clone());

        let cover_data = doc.get_cover().map(|(data, _)| data);

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let medium = detect_medium(&ext);

        Ok(BookMeta {
            title,
            subtitle: None,
            authors,
            isbn,
            publisher,
            language,
            published_date,
            description: description.filter(|d| !d.is_empty()),
            page_count: None,
            cover_data,
            medium,
            format: ext,
        })
    }
}
