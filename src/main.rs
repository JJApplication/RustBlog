mod config;
mod db;
mod error;
mod handlers;
mod logger;
mod middleware;
mod models;
mod router;
mod services;
mod state;
mod utils;

use std::sync::Arc;
use tracing::info;

use crate::{config::AppConfig, error::AppError, state::AppState};

/// 应用入口
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Arc::new(AppConfig::load_from_file("config.toml")?);
    logger::init_logging(config.debug, &config.log);
    let db = db::init_db(&config).await?;
    let state = AppState { db, config };
    let app = router::app::build_app(state.clone());

    let addr = format!(
        "{}:{}",
        state.config.server.host, state.config.server.http_port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!(
        "[{}] RustBlog API listening on http://{}",
        state.config.log.prefix,
        addr
    );
    axum::serve(listener, app).await?;
    Ok(())
}
