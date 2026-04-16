use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 单文章查询参数
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    /// 文章唯一名
    pub name: String,
}

/// 单文章响应
#[derive(Debug, Serialize)]
pub struct PostDetail {
    /// 唯一名
    pub name: String,
    /// 标题
    pub title: String,
    /// 日期
    pub date: String,
    /// 摘要
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    /// 正文
    pub content: String,
    /// 标签
    pub tags: String,
    /// 分类
    pub categories: String,
    /// 锁定状态
    pub lock: Option<i32>,
}

/// 获取单篇文章
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<PostQuery>,
) -> Result<impl IntoResponse, AppError> {
    let row = post::Entity::find()
        .filter(post::Column::Name.eq(q.name))
        .one(&state.db)
        .await?;
    let detail = row.map(|v| PostDetail {
        name: v.name,
        title: v.title,
        date: v.date,
        abstract_text: v.abstract_text,
        content: v.content,
        tags: v.tags,
        categories: v.categories,
        lock: v.lock,
    });
    Ok(ok(detail))
}
