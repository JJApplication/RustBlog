use axum::{
    extract::Request,
    extract::State,
    http::{header::AUTHORIZATION, Method},
    middleware::Next,
    response::Response,
};

use crate::{error::AppError, state::AppState, utils::jwt::verify_token};

/// JWT鉴权中间件
pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = request.uri().path();
    if !path.starts_with("/api/dashboard") {
        return Ok(next.run(request).await);
    }

    if request.method() == Method::OPTIONS {
        return Ok(next.run(request).await);
    }

    let token = extract_bearer_token(&request).ok_or(AppError::Unauthorized)?;
    let _claims = verify_token(token, &state.config.admin.jwt_secret)?;
    Ok(next.run(request).await)
}

/// 提取Bearer令牌
pub fn extract_bearer_token<B>(request: &axum::http::Request<B>) -> Option<&str> {
    let value = request.headers().get(AUTHORIZATION)?.to_str().ok()?;
    value.strip_prefix("Bearer ")
}
