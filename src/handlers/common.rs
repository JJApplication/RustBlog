use axum::Json;
use serde::Serialize;

/// 统一API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// 业务码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

/// 构建成功响应
pub fn ok<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        code: 200,
        msg: "success".to_string(),
        data: Some(data),
    })
}
