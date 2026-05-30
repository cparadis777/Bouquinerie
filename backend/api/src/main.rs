use std::net::SocketAddr;
use std::path::Path;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::health_check),
    info(
        title = "Boilerplate API",
        version = "0.1.0",
        description = "A simple Axum API server"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=info,tower_http=info".into()),
        )
        .init();

    let frontend_dist = std::env::var("FRONTEND_DIST").unwrap_or_else(|_| {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        format!("{crate_dir}/../../frontend/build/web")
    });

    info!("frontend dist path: {frontend_dist}");
    let path = Path::new(&frontend_dist);
    if path.exists() {
        info!("frontend dist directory exists, contains index.html: {}", path.join("index.html").exists());
    } else {
        info!("frontend dist directory does not exist");
    }

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .fallback_service(
            ServeDir::new(&frontend_dist)
                .append_index_html_on_directories(true)
                .fallback(ServeFile::new(format!("{frontend_dist}/index.html"))),
        );

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
