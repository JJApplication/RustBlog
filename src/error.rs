use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// 统一错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("io错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("配置解析错误: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("数据库错误: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("JWT错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("未授权")]
    Unauthorized,
    #[error("参数错误: {0}")]
    BadRequest(String),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: i32,
    msg: String,
    data: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, 401),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, 400),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, 500),
        };

        let body = ErrorBody {
            code,
            msg: self.to_string(),
            data: None,
        };
        (status, Json(body)).into_response()
    }
}
