use axum::{extract::State, response::IntoResponse};
use sea_orm::{EntityTrait, Order, QueryOrder};
use std::collections::BTreeMap;

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 获取归档列表
pub async fn handle(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = post::Entity::find()
        .order_by(post::Column::Date, Order::Desc)
        .all(&state.db)
        .await?;
    let mut archive: BTreeMap<String, usize> = BTreeMap::new();
    for item in rows {
        let key = item.date.get(0..7).unwrap_or(&item.date).to_string();
        *archive.entry(key).or_insert(0) += 1;
    }
    Ok(ok(archive))
}
