use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::comment, state::AppState};

#[derive(Debug, Deserialize)]
pub struct CommentQuery {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentBody {
    pub id: i32,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteCommentBody {
    pub id: i32,
}

/// 获取评论（受保护）
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<CommentQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = if let Some(name) = q.name {
        comment::Entity::find()
            .filter(comment::Column::Name.eq(name))
            .all(&state.db)
            .await?
    } else {
        comment::Entity::find().all(&state.db).await?
    };
    Ok(ok(serde_json::json!({"msg":"get comment success","data":rows})))
}

/// 更新评论（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<UpdateCommentBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = comment::Entity::find_by_id(body.id).one(&state.db).await? {
        let mut active: comment::ActiveModel = row.into();
        active.comment = Set(body.comment);
        active.update(&state.db).await?;
        return Ok(ok(serde_json::json!({"msg":"update comment success","data":"success"})));
    }
    Ok(ok(serde_json::json!({"msg":"update comment failed","data":"fail"})))
}

/// 删除评论（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeleteCommentBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = comment::Entity::find_by_id(body.id).one(&state.db).await? {
        let active: comment::ActiveModel = row.into();
        active.delete(&state.db).await?;
        return Ok(ok(serde_json::json!({"msg":"delete comment success","data":"success"})));
    }
    Ok(ok(serde_json::json!({"msg":"delete comment failed","data":"fail"})))
}
