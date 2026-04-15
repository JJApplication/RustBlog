use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 获取订阅（受保护）
pub async fn get() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"get subscribe success","data":[]}))
}

/// 更新订阅（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"update subscribe success","data":"success"}))
}

/// 删除订阅（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"delete subscribe success","data":"success"}))
}
