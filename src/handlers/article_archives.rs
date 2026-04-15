use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 归档文章查询参数
#[derive(Debug, Deserialize)]
pub struct ArchivesQuery {
    /// 年月，如 2026-04
    pub date: String,
}

/// 获取某个归档下的文章
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<ArchivesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = post::Entity::find()
        .filter(post::Column::Date.contains(&q.date))
        .all(&state.db)
        .await?;
    Ok(ok(rows))
}
