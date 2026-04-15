use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::view, state::AppState};

#[derive(Debug, Deserialize)]
pub struct UpsertViewBody {
    pub name: String,
    pub view: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteViewBody {
    pub name: String,
}

/// 获取访问量（受保护）
pub async fn get(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = view::Entity::find().all(&state.db).await?;
    Ok(ok(serde_json::json!({"msg":"get view success","data":rows})))
}

/// 更新访问量（受保护）
pub async fn put(
    State(state): State<AppState>,
    Json(body): Json<UpsertViewBody>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(row) = view::Entity::find()
        .filter(view::Column::Name.eq(body.name.clone()))
        .one(&state.db)
        .await?
    {
        let mut active: view::ActiveModel = row.into();
        active.view = Set(body.view);
        active.update(&state.db).await?;
    } else {
        view::ActiveModel {
            name: Set(body.name),
            view: Set(body.view),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }
    Ok(ok(serde_json::json!({"msg":"update view success","data":"success"})))
}

/// 删除访问量（受保护）
pub async fn delete(
    State(state): State<AppState>,
    Json(body): Json<DeleteViewBody>,
) -> Result<impl IntoResponse, AppError> {
    let rows = view::Entity::find()
        .filter(view::Column::Name.eq(body.name))
        .all(&state.db)
        .await?;
    for row in rows {
        let active: view::ActiveModel = row.into();
        active.delete(&state.db).await?;
    }
    Ok(ok(serde_json::json!({"msg":"delete view success","data":"success"})))
}
