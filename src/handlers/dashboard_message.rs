use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::message, state::AppState};

#[derive(Debug, Deserialize)]
pub struct UpdateMessageBody {
    pub id: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteMessageBody {
    pub id: i32,
}

/// 获取留言（受保护）
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = message::Entity::find().all(&state.db).await?;
    Ok(ok(serde_json::json!({"msg":"get message success","data":rows})))
}

/// 更新留言（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<UpdateMessageBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = message::Entity::find_by_id(body.id).one(&state.db).await? {
        let mut active: message::ActiveModel = row.into();
        active.message = Set(body.message);
        active.update(&state.db).await?;
        return Ok(ok(serde_json::json!({"msg":"update message success","data":"success"})));
    }
    Ok(ok(serde_json::json!({"msg":"update message failed","data":"fail"})))
}

/// 删除留言（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeleteMessageBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = message::Entity::find_by_id(body.id).one(&state.db).await? {
        let active: message::ActiveModel = row.into();
        active.delete(&state.db).await?;
        return Ok(ok(serde_json::json!({"msg":"delete message success","data":"success"})));
    }
    Ok(ok(serde_json::json!({"msg":"delete message failed","data":"fail"})))
}
