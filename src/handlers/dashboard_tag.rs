use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::tag, state::AppState};

#[derive(Debug, Deserialize)]
pub struct TagBody {
    pub tag: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagQuery {
    pub tag: Option<String>,
}

/// 新建标签（受保护）
pub async fn post(
    State(state): State<AppState>,
    Json(body): Json<TagBody>,
) -> Result<impl IntoResponse, AppError> {
    tag::ActiveModel {
        tag: Set(body.tag),
        name: Set(body.name),
        ..Default::default()
    }
    .insert(&state.db)
    .await?;
    Ok(ok("success"))
}

/// 获取标签（受保护）
pub async fn get(
    State(state): State<AppState>,
    Query(q): Query<TagQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = if let Some(tag_name) = q.tag {
        tag::Entity::find()
            .filter(tag::Column::Tag.eq(tag_name))
            .all(&state.db)
            .await?
    } else {
        tag::Entity::find().all(&state.db).await?
    };
    Ok(ok(rows))
}

/// 更新标签（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<TagBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = tag::Entity::find()
        .filter(tag::Column::Name.eq(body.name.clone()))
        .one(&state.db)
        .await?
    {
        let mut active: tag::ActiveModel = row.into();
        active.tag = Set(body.tag);
        active.update(&state.db).await?;
        return Ok(ok("success"));
    }
    Ok(ok("fail"))
}

/// 删除标签（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<TagBody>,
) -> Result<impl IntoResponse, AppError> {
    let rows = tag::Entity::find()
        .filter(tag::Column::Tag.eq(body.tag))
        .filter(tag::Column::Name.eq(body.name))
        .all(&state.db)
        .await?;
    for row in rows {
        let active: tag::ActiveModel = row.into();
        active.delete(&state.db).await?;
    }
    Ok(ok("success"))
}
