use axum::{extract::State, response::IntoResponse};
use sea_orm::EntityTrait;
use std::collections::BTreeSet;

use crate::{error::AppError, handlers::common::ok, models::tag, state::AppState};

/// 获取全部标签
pub async fn handle(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = tag::Entity::find().all(&state.db).await?;
    let tags = rows
        .into_iter()
        .map(|v| v.tag)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(ok(tags))
}
