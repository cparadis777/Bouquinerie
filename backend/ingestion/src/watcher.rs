use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use notify::{Event, EventKind, RecursiveMode, Watcher};
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use tracing::{error, info, instrument, warn};

use crate::config::Config;
use crate::processor;

#[instrument(skip(db))]
pub async fn start(db: DatabaseConnection, config: Config) {
    config.validate();

    let pending: Arc<Mutex<HashMap<PathBuf, tokio::task::JoinHandle<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<PathBuf>();

    let watch_cfg = config.clone();
    let event_tx = tx.clone();
    let mut watcher = match notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                    for path in event.paths {
                        if is_supported(&path, &watch_cfg.supported_formats) {
                            let _ = event_tx.send(path);
                        }
                    }
                }
            }
            Err(e) => warn!("notify error: {e}"),
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            error!("failed to create file watcher: {e}");
            return;
        }
    };

    for dir in &config.watched_dirs {
        match watcher.watch(dir, RecursiveMode::NonRecursive) {
            Ok(()) => info!("watching directory: {}", dir.display()),
            Err(e) => warn!("failed to watch {}: {e}", dir.display()),
        }
    }

    scan_existing(&config.watched_dirs, &config.supported_formats, &tx).await;

    let db = Arc::new(db);
    let config = Arc::new(config);
    let debounce_secs = config.debounce_secs;

    // Event receiver: abort previous handle, spawn new debounced timer
    let pending_clone = pending.clone();
    let db_clone = db.clone();
    let config_clone = config.clone();
    tokio::spawn(async move {
        while let Some(path) = rx.recv().await {
            let mut map = pending_clone.lock().await;
            if let Some(handle) = map.remove(&path) {
                handle.abort();
            }
            let p = pending_clone.clone();
            let db = db_clone.clone();
            let config = config_clone.clone();
            let path_for_task = path.clone();
            let path_for_cleanup = path.clone();
            let handle = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(debounce_secs)).await;
                process_file(&db, &config, &path_for_task).await;
                p.lock().await.remove(&path_for_cleanup);
            });
            map.insert(path, handle);
        }
    });

    // Keep watcher alive until Ctrl+C
    let _watcher = watcher;
    tokio::signal::ctrl_c().await.ok();
    info!("file watcher shutting down");
}

#[instrument(skip(tx))]
async fn scan_existing(
    dirs: &[PathBuf],
    formats: &[String],
    tx: &tokio::sync::mpsc::UnboundedSender<PathBuf>,
) {
    for dir in dirs {
        if !dir.exists() {
            info!("scan directory does not exist: {}", dir.display());
            continue;
        }
        let mut entries = match tokio::fs::read_dir(dir).await {
            Ok(e) => e,
            Err(e) => {
                warn!("failed to read directory {}: {e}", dir.display());
                continue;
            }
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_file() && is_supported(&path, formats) {
                let _ = tx.send(path);
            }
        }
    }
}

fn is_supported(path: &Path, formats: &[String]) -> bool {
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return false,
    };
    formats.iter().any(|f| f == &ext)
}

#[instrument(skip(db, config))]
async fn process_file(db: &DatabaseConnection, config: &Config, path: &Path) {
    if !path.exists() {
        info!("file disappeared before processing");
        return;
    }

    match processor::ingest(db, config, path).await {
        Ok(result) => {
            info!(
                book_id = %result.book_id,
                "ingested successfully"
            );
        }
        Err(processor::ProcessError::Duplicate(_)) => {
            info!("duplicate file, moving to _duplicates/");
            let dest = config.library_path.join("_duplicates");
            let _ = tokio::fs::create_dir_all(&dest).await;
            let _ = tokio::fs::rename(path, &dest.join(path.file_name().unwrap_or_default())).await;
        }
        Err(e) => {
            error!(error = %e, "failed to ingest");
            let dest = config.library_path.join("_failed");
            let _ = tokio::fs::create_dir_all(&dest).await;
            let _ = tokio::fs::rename(path, &dest.join(path.file_name().unwrap_or_default())).await;
        }
    }
}
