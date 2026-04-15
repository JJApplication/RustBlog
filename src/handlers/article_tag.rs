use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use std::collections::BTreeSet;

use crate::{error::AppError, handlers::common::ok, models::{post, tag}, state::AppState};

/// 标签文章查询参数
#[derive(Debug, Deserialize)]
pub struct TagQuery {
    /// 标签名
    pub tag: String,
}

/// 按标签获取文章
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<TagQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = tag::Entity::find()
        .filter(tag::Column::Tag.eq(q.tag))
        .all(&state.db)
        .await?;
    let names = rows.into_iter().map(|v| v.name).collect::<BTreeSet<_>>();
    let mut list = Vec::new();
    for name in names {
        if let Some(p) = post::Entity::find()
            .filter(post::Column::Name.eq(name))
            .one(&state.db)
            .await?
        {
            list.push(p);
        }
    }
    Ok(ok(list))
}
