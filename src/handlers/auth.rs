use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap},
    response::IntoResponse,
};

use crate::{handlers::common::ok, state::AppState, utils::jwt::verify_token};

/// 鉴权检查接口（始终返回200）
/// - token有效: data=1
/// - token无效: data=0
pub async fn check(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let token_raw = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(str::trim);

    if let Some(raw) = token_raw {
        let token = raw
            .strip_prefix("Bearer ")
            .or_else(|| raw.strip_prefix("bearer "))
            .unwrap_or(raw);
        if !token.is_empty() && verify_token(token, &state.config.admin.jwt_secret).is_ok() {
            return ok(1);
        }
    }
    ok(0)
}
