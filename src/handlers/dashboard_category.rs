use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 获取分类（受保护）
pub async fn get() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"get category success","data":[]}))
}

/// 更新分类（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"update category success","data":"success"}))
}

/// 删除分类（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"delete category success","data":"success"}))
}
