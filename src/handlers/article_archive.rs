use axum::{extract::State, response::IntoResponse};
use sea_orm::{EntityTrait, Order, QueryOrder};
use serde::Serialize;
use std::collections::BTreeMap;

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 归档项
#[derive(Debug, Serialize)]
pub struct ArchiveItem {
    /// 年月，例如 2026-04
    pub date: String,
    /// 数量
    pub count: usize,
}

/// 获取归档列表
pub async fn handle(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rows = post::Entity::find()
        .order_by(post::Column::Date, Order::Desc)
        .all(&state.db)
        .await?;

    let mut flag_map: BTreeMap<String, usize> = BTreeMap::new();
    for item in rows {
        let parts = item.date.split('-').collect::<Vec<_>>();
        if parts.len() > 1 {
            let ym = format!("{}-{}", parts[0], parts[1]);
            *flag_map.entry(ym).or_insert(0) += 1;
        }
    }

    let data = flag_map
        .into_iter()
        .map(|(date, count)| ArchiveItem { date, count })
        .collect::<Vec<_>>();

    Ok(ok(data))
}
