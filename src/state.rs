use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::config::AppConfig;

/// 全局应用状态
#[derive(Clone)]
pub struct AppState {
    /// 数据库连接
    pub db: DatabaseConnection,
    /// 配置对象
    pub config: Arc<AppConfig>,
}
