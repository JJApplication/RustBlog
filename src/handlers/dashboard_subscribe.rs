use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 获取订阅（受保护）
pub async fn get() -> impl IntoResponse {
    ok(Vec::<String>::new())
}

/// 更新订阅（受保护）
pub async fn put() -> impl IntoResponse {
    ok("success")
}

/// 删除订阅（受保护）
pub async fn delete() -> impl IntoResponse {
    ok("success")
}
