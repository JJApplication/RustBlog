use axum::{extract::State, response::IntoResponse, Json};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, Order, QueryOrder};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::message, state::AppState};

/// 新增留言请求体
#[derive(Debug, Deserialize)]
pub struct SaveMessageBody {
    /// 留言内容（兼容老接口 mes）
    pub mes: String,
}

/// 获取留言列表
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = message::Entity::find()
        .order_by(message::Column::Id, Order::Desc)
        .all(&state.db)
        .await?;
    Ok(ok(rows))
}

/// 新增留言
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<SaveMessageBody>,
) -> Result<impl IntoResponse, AppError> {
    let active = message::ActiveModel {
        user: Set("anonymous".to_string()),
        date: Set(Local::now().format("%Y-%m-%d").to_string()),
        message: Set(body.mes),
        ..Default::default()
    };
    active.insert(&state.db).await?;
    Ok(ok("saved"))
}
