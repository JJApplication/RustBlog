use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 新建文章（受保护）
pub async fn post() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_post_create"}))
}

/// 更新文章（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_post_update"}))
}

/// 删除文章（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_post_delete"}))
}
