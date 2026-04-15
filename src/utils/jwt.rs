use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// JWT声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 主题（用户标识）
    pub sub: String,
    /// 过期时间戳（秒）
    pub exp: usize,
}

/// 生成JWT令牌
pub fn generate_token(sub: &str, secret: &str, expire_hours: i64) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::hours(expire_hours)).timestamp() as usize;
    let claims = Claims {
        sub: sub.to_string(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

/// 验证JWT令牌
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
