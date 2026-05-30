use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    let filter = if let Ok(level) = std::env::var("LOG_LEVEL") {
        EnvFilter::new(&level)
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("api=info,tower_http=info"))
    };

    let fmt = tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(filter);

    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        fmt.json().flatten_event(true).init();
    } else {
        fmt.compact().init();
    }
}
