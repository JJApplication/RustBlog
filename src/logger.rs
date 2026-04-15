use tracing_subscriber::EnvFilter;

use crate::config::LogConfig;

/// 初始化日志系统
pub fn init_logging(debug: bool, log_cfg: &LogConfig) {
    let filter = EnvFilter::try_new(log_cfg.level.clone()).unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(log_cfg.color)
        .with_target(debug)
        .compact()
        .init();
}
