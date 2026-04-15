use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::view, state::AppState};

/// 访问量查询参数
#[derive(Debug, Deserialize)]
pub struct ViewQuery {
    /// 文章名
    pub name: String,
}

/// 获取访问量
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<ViewQuery>,
) -> Result<impl IntoResponse, AppError> {
    let row = view::Entity::find()
        .filter(view::Column::Name.eq(q.name))
        .one(&state.db)
        .await?;
    Ok(ok(row.map(|v| v.view).unwrap_or(0)))
}
