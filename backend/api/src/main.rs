use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use axum::{routing::get, Router};
use db::{init_database, run_migrations, state::AppState};
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};
use uuid::Uuid;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod health;
mod handlers;
mod logging;
mod response;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        handlers::books::list_books,
        handlers::books::get_book,
        handlers::authors::list_authors,
        handlers::series::list_series,
    ),
    components(schemas(
        db::entities::books::Model,
        db::entities::authors::Model,
        db::entities::series::Model,
        db::entities::identifiers::Model,
        response::BookListResponse,
        response::BookListEntry,
        response::AuthorListResponse,
        response::SeriesListResponse,
        response::BookResponse,
    )),
    info(
        title = "Bouquinerie",
        version = "0.1.0",
        description = "Ebook management platform"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    logging::init_logging();
    dotenvy::dotenv().ok();

    let frontend_dist = std::env::var("FRONTEND_DIST").unwrap_or_else(|_| {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        format!("{crate_dir}/../../web/dist")
    });

    info!("frontend dist path: {frontend_dist}");
    let path = Path::new(&frontend_dist);
    if path.exists() {
        info!(
            "frontend dist directory exists, contains index.html: {}",
            path.join("index.html").exists()
        );
    } else {
        info!("frontend dist directory does not exist");
    }

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment");
    let db = init_database(&database_url).await;
    run_migrations(&db).await;

    let state = AppState { db: db.clone() };

    let ingestion_cfg = ingestion::config::Config::from_env();
    info!(
        watched_dirs = ?ingestion_cfg.watched_dirs,
        library_path = %ingestion_cfg.library_path.display(),
        debounce_secs = ingestion_cfg.debounce_secs,
        supported_formats = ?ingestion_cfg.supported_formats,
        "ingestion config loaded"
    );
    let watcher_db = db.clone();
    tokio::spawn(async move {
        ingestion::watcher::start(watcher_db, ingestion_cfg).await;
    });

    let app = Router::new()
        .route("/health", get(health::health_check))
        .route("/api/books", get(handlers::books::list_books))
        .route("/api/books/{id}", get(handlers::books::get_book))
        .route("/api/authors", get(handlers::authors::list_authors))
        .route("/api/series", get(handlers::series::list_series))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<_>| {
                    let request_id = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| Uuid::new_v4().to_string());
                    info_span!(
                        "http_request",
                        method = %req.method(),
                        uri = %req.uri().path(),
                        request_id = %request_id,
                    )
                })
                .on_response(
                    |res: &axum::http::Response<_>, latency: Duration, _span: &Span| {
                        let status = res.status().as_u16();
                        let latency_ms = latency.as_millis() as u64;
                        if status >= 500 {
                            tracing::error!(status, latency_ms);
                        } else if status >= 400 {
                            tracing::warn!(status, latency_ms);
                        } else {
                            tracing::info!(status, latency_ms);
                        }
                    },
                )
                .on_failure(
                    |_err: ServerErrorsFailureClass,
                     latency: Duration,
                     _span: &Span| {
                        tracing::error!(
                            latency_ms = latency.as_millis() as u64,
                            "request failed"
                        );
                    },
                ),
        )
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(CorsLayer::permissive())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let app = if path.exists() {
        info!("serving frontend from {frontend_dist}");
        app.fallback_service(
            ServeDir::new(&frontend_dist)
                .append_index_html_on_directories(true)
                .fallback(ServeFile::new(format!("{frontend_dist}/index.html"))),
        )
    } else {
        info!("no frontend dist found at {frontend_dist}, API only");
        app
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server running on http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    info!("Shutting down gracefully...");
}
