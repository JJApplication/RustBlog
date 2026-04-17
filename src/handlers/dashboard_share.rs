use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::share, state::AppState};

#[derive(Debug, Deserialize)]
pub struct UpsertShareBody {
    pub name: String,
    pub share: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteShareBody {
    pub name: String,
}

/// 获取分享（受保护）
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = share::Entity::find().all(&state.db).await?;
    Ok(ok(rows))
}

/// 更新分享（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<UpsertShareBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = share::Entity::find()
        .filter(share::Column::Name.eq(body.name.clone()))
        .one(&state.db)
        .await?
    {
        let mut active: share::ActiveModel = row.into();
        active.share = Set(body.share);
        active.update(&state.db).await?;
    } else {
        share::ActiveModel {
            name: Set(body.name),
            share: Set(body.share),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok("success"))
}

/// 删除分享（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeleteShareBody>,
) -> Result<impl IntoResponse, AppError> {
    let rows = share::Entity::find()
        .filter(share::Column::Name.eq(body.name))
        .all(&state.db)
        .await?;
    for row in rows {
        let active: share::ActiveModel = row.into();
        active.delete(&state.db).await?;
    }
    Ok(ok("success"))
}
