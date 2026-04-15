use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 获取专栏（受保护）
pub async fn get() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"get zhuanlan success","data":[]}))
}

/// 更新专栏（受保护）
pub async fn put() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"update zhuanlan success","data":"success"}))
}

/// 删除专栏（受保护）
pub async fn delete() -> impl IntoResponse {
    ok(serde_json::json!({"msg":"delete zhuanlan success","data":"success"}))
}
