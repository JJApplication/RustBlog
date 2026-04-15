use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 新建标签（受保护）
pub async fn post() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_tag_create"}))
}

/// 更新标签（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_tag_update"}))
}

/// 删除标签（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"ok": true, "action": "dashboard_tag_delete"}))
}
