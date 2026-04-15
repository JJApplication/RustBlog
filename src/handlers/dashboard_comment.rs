use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 更新评论（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_comment_update"}))
}

/// 删除评论（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_comment_delete"}))
}
