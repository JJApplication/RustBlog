use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    handlers::common::ok,
    state::AppState,
    utils::jwt::generate_token,
};

/// 登录请求体
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// 用户名
    pub name: String,
    /// 密码
    pub passwd: String,
}

/// 登录响应体
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// JWT令牌
    pub token: String,
}

/// 管理员登录接口
pub async fn handle(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    if payload.name != state.config.admin.username || payload.passwd != state.config.admin.password {
        return Err(AppError::Unauthorized);
    }
    let token = generate_token(
        &payload.name,
        &state.config.admin.jwt_secret,
        state.config.admin.jwt_expire_hours,
    )?;
    Ok(ok(LoginResponse { token }))
}
