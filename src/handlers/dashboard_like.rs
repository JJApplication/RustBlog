use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::like, state::AppState};

#[derive(Debug, Deserialize)]
pub struct UpsertLikeBody {
    pub name: String,
    pub like: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteLikeBody {
    pub name: String,
}

/// 获取点赞（受保护）
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = like::Entity::find().all(&state.db).await?;
    Ok(ok(serde_json::json!({"msg":"get like success","data":rows})))
}

/// 更新点赞（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<UpsertLikeBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = like::Entity::find()
        .filter(like::Column::Name.eq(body.name.clone()))
        .one(&state.db)
        .await?
    {
        let mut active: like::ActiveModel = row.into();
        active.like = Set(body.like);
        active.update(&state.db).await?;
    } else {
        like::ActiveModel {
            name: Set(body.name),
            like: Set(body.like),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok(serde_json::json!({"msg":"update like success","data":"success"})))
}

/// 删除点赞（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeleteLikeBody>,
) -> Result<impl IntoResponse, AppError> {
    let rows = like::Entity::find()
        .filter(like::Column::Name.eq(body.name))
        .all(&state.db)
        .await?;
    for row in rows {
        let active: like::ActiveModel = row.into();
        active.delete(&state.db).await?;
    }
    Ok(ok(serde_json::json!({"msg":"delete like success","data":"success"})))
}
