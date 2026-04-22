use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::share, state::AppState};

/// 分享查询参数
#[derive(Debug, Deserialize)]
pub struct ShareQuery {
    /// 文章名
    pub name: String,
}

/// 获取分享数
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<ShareQuery>,
) -> Result<impl IntoResponse, AppError> {
    let row = share::Entity::find()
        .filter(share::Column::Name.eq(q.name))
        .one(&state.db)
        .await?;
    Ok(ok(row.map(|v| v.share).unwrap_or(0)))
}

/// 增加分享数
pub async fn post(
    State(state): State<AppState>,
    Query(q): Query<ShareQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(v) = share::Entity::find()
        .filter(share::Column::Name.eq(&q.name))
        .one(&state.db)
        .await?
    {
        let next_share = v.share + 1;
        let mut active: share::ActiveModel = v.into();
        active.share = Set(next_share);
        active.update(&state.db).await?;
    } else {
        share::ActiveModel {
            name: Set(q.name),
            share: Set(1),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok("success"))
}
