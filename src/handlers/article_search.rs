use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 搜索参数
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// 关键词（兼容key/keyword）
    pub key: Option<String>,
    /// 关键词
    pub keyword: Option<String>,
}

/// 搜索文章
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let kw = q.key.or(q.keyword).unwrap_or_default();
    if kw.is_empty() {
        return Ok(ok(Vec::<post::Model>::new()));
    }

    let rows = post::Entity::find()
        .filter(
            Condition::any()
                .add(post::Column::Name.contains(&kw))
                .add(post::Column::Title.contains(&kw))
                .add(post::Column::Content.contains(&kw))
                .add(post::Column::Tags.contains(&kw)),
        )
        .all(&state.db)
        .await?;
    Ok(ok(rows))
}
