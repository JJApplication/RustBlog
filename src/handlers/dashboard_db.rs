use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 数据库初始化（受保护）
pub async fn init() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"db init","data":"success"}))
}

/// 数据库备份（受保护）
pub async fn backup() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"backup db success","data":"success"}))
}

/// 数据库导出（受保护）
pub async fn export() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"export db success","data":"success"}))
}
