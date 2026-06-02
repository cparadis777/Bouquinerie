use std::path::PathBuf;

use tracing::warn;

#[derive(Clone, Debug)]
pub struct Config {
    pub watched_dirs: Vec<PathBuf>,
    pub library_path: PathBuf,
    pub debounce_secs: u64,
    pub supported_formats: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let watched = std::env::var("WATCHED_DIRS").unwrap_or_default();
        let watched_dirs: Vec<PathBuf> = if watched.is_empty() {
            vec![PathBuf::from("/incoming")]
        } else {
            watched
                .split(',')
                .map(|s| PathBuf::from(s.trim()))
                .collect()
        };

        let library_path = std::env::var("LIBRARY_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/library"));

        let debounce_secs = std::env::var("INGEST_DEBOUNCE_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let formats = std::env::var("SUPPORTED_FORMATS")
            .unwrap_or_else(|_| "epub,pdf,mobi,azw3,m4b,mp3".to_string());
        let supported_formats: Vec<String> = formats
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();

        Self {
            watched_dirs,
            library_path,
            debounce_secs,
            supported_formats,
        }
    }

    pub fn validate(&self) {
        if self.supported_formats.is_empty() {
            warn!("SUPPORTED_FORMATS is empty, no files will be processed");
        }
        for dir in &self.watched_dirs {
            if !dir.exists() {
                warn!("watched directory does not exist: {}", dir.display());
            }
        }
        if !self.library_path.exists() {
            warn!(
                "library path does not exist, will attempt to create: {}",
                self.library_path.display()
            );
        }
    }
}
