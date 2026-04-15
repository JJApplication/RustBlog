use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 更新留言（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_message_update"}))
}

/// 删除留言（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_message_delete"}))
}
