use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::like, state::AppState};

/// 点赞查询参数
#[derive(Debug, Deserialize)]
pub struct LikeQuery {
    /// 文章名
    pub name: String,
}

/// 点赞请求体
#[derive(Debug, Deserialize)]
pub struct AddLikeBody {
    /// 文章名
    pub name: String,
}

/// 获取点赞数
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<LikeQuery>,
) -> Result<impl IntoResponse, AppError> {
    let row = like::Entity::find()
        .filter(like::Column::Name.eq(q.name))
        .one(&state.db)
        .await?;
    Ok(ok(row.map(|v| v.like).unwrap_or(0)))
}

/// 新增点赞
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<AddLikeBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(v) = like::Entity::find()
        .filter(like::Column::Name.eq(&body.name))
        .one(&state.db)
        .await?
    {
        let next_like = v.like + 1;
        let mut active: like::ActiveModel = v.into();
        active.like = Set(next_like);
        active.update(&state.db).await?;
    } else {
        like::ActiveModel {
            name: Set(body.name),
            like: Set(1),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok("success"))
}
