use axum::response::IntoResponse;

use crate::handlers::common::ok;

/// 健康检查接口
pub async fn handle() -> impl IntoResponse {
    ok(serde_json::json!({"message":"hello this is rust blog"}))
}
