use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 获取专栏（受保护）
pub async fn get() -> impl IntoResponse {
    ok(Vec::<String>::new())
}

/// 更新专栏（受保护）
pub async fn put() -> impl IntoResponse {
    ok("success")
}

/// 删除专栏（受保护）
pub async fn delete() -> impl IntoResponse {
    ok("success")
}
