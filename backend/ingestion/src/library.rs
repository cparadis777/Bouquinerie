use itertools::Itertools;
use std::path::{Component, Path, PathBuf};

use sha2::{Digest, Sha256};
use tokio::fs;
use tokio::io::AsyncReadExt;

use crate::metadata::BookMeta;

const SUFFIXES: [&str; 12] = [
    "Jr.", "Sr.", "II", "III", "IV", "V", "M.D.", "Ph.D.", "Esq.", "CPA", "DDS", "MD",
];

#[derive(Debug, thiserror::Error)]
pub enum LibraryError {
    #[error("failed to read file: {0}")]
    ReadFailed(std::io::Error),
    #[error("failed to write file: {0}")]
    WriteFailed(std::io::Error),
    #[error("hash mismatch after copy")]
    HashMismatch,
    #[error("path validation failed: {0}")]
    PathValidation(String),
}

pub fn sort_name(name: &str) -> String {
    let name = name.trim();
    if name.contains(',') {
        return name.to_string();
    }
    let parts: Vec<&str> = name.split_whitespace().collect();
    match parts.len() {
        0 => "Unknown".to_string(),
        1 => parts[0].to_string(),
        _ => {
            let suffixes: Vec<&str> = parts
                .iter()
                .rev()
                .take_while(|w| SUFFIXES.contains(w))
                .copied()
                .collect();
            let suffix_count = suffixes.len();
            let surname_end = parts.len() - suffix_count;
            if surname_end == 0 {
                return name.to_string();
            }
            let last = parts[surname_end - 1];
            let first = &parts[..surname_end - 1];
            let surname = if suffix_count > 0 {
                let mut s = vec![last];
                s.extend(suffixes.iter().rev().copied());
                s.join(" ")
            } else {
                last.to_string()
            };
            format!("{}, {}", surname, first.join(" "))
        }
    }
}

pub fn sanitize_filename(name: &str) -> String {
    let illegal = ['/', '\\', '\0', ':', '*', '?', '"', '<', '>', '|'];
    let mut sanitized: String = name
        .chars()
        .map(|c| {
            if illegal.contains(&c) || c.is_control() {
                ' '
            } else {
                c
            }
        })
        .collect();

    sanitized = sanitized.split_whitespace().join(" ");
    sanitized.truncate(200);
    sanitized.trim().to_string()
}

fn validate_rel_path(path: &Path) -> Result<(), LibraryError> {
    for component in path.components() {
        match component {
            Component::ParentDir => {
                return Err(LibraryError::PathValidation(
                    "parent dir traversal not allowed".into(),
                ));
            }
            Component::RootDir => {
                return Err(LibraryError::PathValidation(
                    "absolute path components not allowed".into(),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn build_rel_path(meta: &BookMeta) -> PathBuf {
    let primary_author = meta
        .authors
        .first()
        .map(|a| sort_name(a))
        .unwrap_or_else(|| "Unknown".to_string());
    let primary_author = sanitize_filename(&primary_author);

    let title = sanitize_filename(&meta.title);

    let year = meta
        .published_date
        .map(|d| d.format("%Y").to_string())
        .unwrap_or_default();

    let book_dir = if year.is_empty() {
        title.to_string()
    } else {
        format!("{} ({})", title, year)
    };

    PathBuf::from(primary_author).join(book_dir)
}

pub async fn hash_file(path: &Path) -> Result<String, LibraryError> {
    let file = tokio::fs::File::open(path)
        .await
        .map_err(LibraryError::ReadFailed)?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 65536];
    loop {
        let n = reader
            .read(&mut buf)
            .await
            .map_err(LibraryError::ReadFailed)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub async fn move_to_library(
    src: &Path,
    library_root: &Path,
    rel_path: &Path,
    filename: &str,
) -> Result<(PathBuf, String), LibraryError> {
    validate_rel_path(rel_path)?;

    let dest_dir = library_root.join(rel_path);
    fs::create_dir_all(&dest_dir)
        .await
        .map_err(LibraryError::WriteFailed)?;

    let dest = dest_dir.join(filename);

    let hash_before = hash_file(src).await?;

    fs::copy(src, &dest)
        .await
        .map_err(LibraryError::WriteFailed)?;

    let hash_after = hash_file(&dest).await?;
    if hash_before != hash_after {
        let _ = fs::remove_file(&dest).await;
        return Err(LibraryError::HashMismatch);
    }

    fs::remove_file(src)
        .await
        .map_err(LibraryError::WriteFailed)?;

    Ok((dest, hash_before))
}

pub async fn save_cover(book_dir: &Path, data: &[u8]) -> Result<PathBuf, LibraryError> {
    let cover_path = book_dir.join("cover.jpg");
    fs::write(&cover_path, data)
        .await
        .map_err(LibraryError::WriteFailed)?;
    Ok(cover_path)
}

pub async fn file_size(path: &Path) -> Result<i64, LibraryError> {
    let metadata = fs::metadata(path).await.map_err(LibraryError::ReadFailed)?;
    Ok(metadata.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{BookMeta, Medium};
    use chrono::NaiveDate;

    #[test]
    fn test_sort_name_simple() {
        assert_eq!(sort_name("Brandon Sanderson"), "Sanderson, Brandon");
    }

    #[test]
    fn test_sort_name_already_sorted() {
        assert_eq!(sort_name("Sanderson, Brandon"), "Sanderson, Brandon");
    }

    #[test]
    fn test_sort_name_multi_word() {
        assert_eq!(
            sort_name("John Ronald Reuel Tolkien"),
            "Tolkien, John Ronald Reuel"
        );
    }

    #[test]
    fn test_sort_name_single() {
        assert_eq!(sort_name("Plato"), "Plato");
    }

    #[test]
    fn test_sort_name_with_suffix() {
        assert_eq!(sort_name("Elon Musk Jr."), "Musk Jr., Elon");
    }

    #[test]
    fn test_sort_name_with_multi_suffix() {
        assert_eq!(
            sort_name("John F. Kennedy Jr. III"),
            "Kennedy Jr. III, John F."
        );
    }

    #[test]
    fn test_sort_name_suffix_only() {
        assert_eq!(sort_name("Jr. Smith"), "Smith, Jr.");
    }

    #[test]
    fn test_sanitize_filename_replaces_slashes() {
        assert_eq!(sanitize_filename("a/b:c"), "a b c");
    }

    #[test]
    fn test_sanitize_filename_collapses_spaces() {
        assert_eq!(sanitize_filename("a  b   c"), "a b c");
    }

    #[test]
    fn test_sanitize_filename_trims() {
        assert_eq!(sanitize_filename("  hello  "), "hello");
    }

    #[test]
    fn test_build_rel_path_with_year() {
        let meta = BookMeta {
            title: "Mistborn".into(),
            subtitle: None,
            authors: vec!["Brandon Sanderson".into()],
            isbn: None,
            publisher: None,
            language: "en".into(),
            published_date: NaiveDate::parse_from_str("2006-07-17", "%Y-%m-%d").ok(),
            description: None,
            page_count: None,
            cover_data: None,
            medium: Medium::Ebook,
            format: "epub".into(),
        };
        let path = build_rel_path(&meta);
        assert_eq!(
            path,
            PathBuf::from("Sanderson, Brandon").join("Mistborn (2006)")
        );
    }

    #[test]
    fn test_build_rel_path_without_year() {
        let meta = BookMeta {
            title: "Untitled".into(),
            subtitle: None,
            authors: vec!["Unknown".into()],
            isbn: None,
            publisher: None,
            language: "en".into(),
            published_date: None,
            description: None,
            page_count: None,
            cover_data: None,
            medium: Medium::Ebook,
            format: "epub".into(),
        };
        let path = build_rel_path(&meta);
        assert_eq!(path, PathBuf::from("Unknown").join("Untitled"));
    }

    #[test]
    fn test_validate_rel_path_rejects_parent() {
        assert!(validate_rel_path(Path::new("../escape")).is_err());
    }

    #[test]
    fn test_validate_rel_path_accepts_normal() {
        assert!(validate_rel_path(Path::new("Author/Book (2020)")).is_ok());
    }
}
