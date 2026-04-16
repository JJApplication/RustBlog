use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    error::AppError,
    handlers::common::ok,
    models::{post, zhuanlan},
    state::AppState,
    utils::content_to_abstract::content_to_abs,
};

/// 专栏列表响应项
#[derive(Debug, Serialize)]
pub struct ZhuanlanListItem {
    /// 链接标识
    pub link: String,
    /// 标题
    pub title: String,
    /// 日期
    pub date: String,
    /// 文章名列表
    pub posts: Vec<String>,
    /// 描述
    pub content: String,
}

/// 专栏文章项
#[derive(Debug, Serialize)]
pub struct ZhuanlanPostItem {
    pub name: String,
    pub title: String,
    pub date: String,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    pub tags: String,
}

/// 专栏详情响应
#[derive(Debug, Serialize)]
pub struct ZhuanlanDetail {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub posts: Vec<ZhuanlanPostItem>,
    pub content: String,
}

/// 获取专栏列表 GET /api/zhuanlan
pub async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = zhuanlan::Entity::find().all(&state.db).await?;
    let data = rows
        .into_iter()
        .map(|z| ZhuanlanListItem {
            link: if z.name.trim().is_empty() {
                z.id.to_string()
            } else {
                z.name
            },
            title: z.title,
            date: z.date,
            posts: split_posts(&z.posts),
            content: z.content,
        })
        .collect::<Vec<_>>();
    Ok(ok(data))
}

/// 获取专栏详情 GET /api/zhuanlan/{link}
pub async fn detail(
    State(state): State<AppState>,
    Path(link): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let row = if let Ok(id) = link.parse::<i32>() {
        zhuanlan::Entity::find_by_id(id).one(&state.db).await?
    } else {
        zhuanlan::Entity::find()
            .filter(zhuanlan::Column::Name.eq(link))
            .one(&state.db)
            .await?
    };

    let data = if let Some(z) = row {
        let mut posts = Vec::new();
        for name in split_posts(&z.posts) {
            if let Some(p) = post::Entity::find()
                .filter(post::Column::Name.eq(name))
                .one(&state.db)
                .await?
            {
                posts.push(ZhuanlanPostItem {
                    name: p.name,
                    title: p.title,
                    date: p.date,
                    abstract_text: content_to_abs(&p.abstract_text, &p.content, &state.config.app),
                    tags: p.tags,
                });
            }
        }
        Some(ZhuanlanDetail {
            id: z.id,
            title: z.title,
            date: z.date,
            posts,
            content: z.content,
        })
    } else {
        None
    };
    Ok(ok(data))
}

fn split_posts(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
}
