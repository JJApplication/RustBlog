use axum::{extract::{Query, State}, response::IntoResponse, Json};
use sea_orm::{EntityTrait, Order, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    models::post,
    state::AppState,
    utils::content_to_abstract::content_to_abs,
};

/// 文章列表查询参数
#[derive(Debug, Deserialize)]
pub struct PostsQuery {
    /// 页码，从1开始
    pub page: Option<usize>,
    /// 每页数量
    #[serde(rename = "pageSize")]
    pub page_size: Option<usize>,
}

/// 文章列表响应项
#[derive(Debug, Serialize)]
pub struct PostListItem {
    /// id
    pub id: i32,
    /// 唯一名
    pub name: String,
    /// 标题
    pub title: String,
    /// 日期
    pub date: String,
    /// 摘要
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    /// 标签
    pub tags: String,
    /// 锁定状态
    pub lock: Option<i32>,
}

/// 文章列表响应
#[derive(Debug, Serialize)]
struct PostsResponse {
    code: i32,
    msg: String,
    data: Vec<PostListItem>,
    len: usize,
}

/// 获取文章列表
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<PostsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = q.page.unwrap_or(1).max(1);
    let page_size = q.page_size.unwrap_or(5).max(1);

    let rows = post::Entity::find()
        .order_by(post::Column::Id, Order::Desc)
        .all(&state.db)
        .await?;

    let start = (page - 1).saturating_mul(page_size);
    let total_len = rows.len();
    let list = rows
        .into_iter()
        .skip(start)
        .take(page_size)
        .map(|v| PostListItem {
            id: v.id,
            name: v.name,
            title: v.title,
            date: v.date,
            abstract_text: content_to_abs(&v.abstract_text, &v.content, &state.config.app),
            tags: v.tags,
            lock: v.lock,
        })
        .collect::<Vec<_>>();
    Ok(Json(PostsResponse {
        code: 200,
        msg: "success".to_string(),
        data: list,
        len: total_len,
    }))
}
