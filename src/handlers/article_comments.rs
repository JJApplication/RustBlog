use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::comment, state::AppState};

/// 评论查询参数
#[derive(Debug, Deserialize)]
pub struct CommentQuery {
    /// 文章名
    pub name: String,
}

/// 新增评论请求体
#[derive(Debug, Deserialize)]
pub struct AddCommentBody {
    /// 文章名
    pub name: String,
    /// 用户
    pub user: String,
    /// 评论内容
    pub comment: String,
}

/// 获取文章评论
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<CommentQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = comment::Entity::find()
        .filter(comment::Column::Name.eq(q.name))
        .all(&state.db)
        .await?;
    Ok(ok(rows))
}

/// 新增评论
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<AddCommentBody>,
) -> Result<impl IntoResponse, AppError> {
    let active = comment::ActiveModel {
        name: Set(body.name),
        user: Set(body.user),
        date: Set(Local::now().format("%Y-%m-%d").to_string()),
        comment: Set(body.comment),
        ..Default::default()
    };
    active.insert(&state.db).await?;
    Ok(ok("success"))
}
