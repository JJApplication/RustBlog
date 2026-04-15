use axum::{extract::{Query, State}, response::IntoResponse};
use sea_orm::{EntityTrait, Order, QueryOrder};
use serde::Deserialize;

use crate::{error::AppError, handlers::common::ok, models::post, state::AppState};

/// 上下篇查询参数
#[derive(Debug, Deserialize)]
pub struct BrotherQuery {
    /// 当前文章名
    pub name: String,
}

/// 获取文章上下篇
pub async fn handle(
    State(state): State<AppState>,
    Query(q): Query<BrotherQuery>,
) -> Result<impl IntoResponse, AppError> {
    let rows = post::Entity::find()
        .order_by(post::Column::Id, Order::Desc)
        .all(&state.db)
        .await?;
    let mut prev = String::new();
    let mut next = String::new();
    for (idx, item) in rows.iter().enumerate() {
        if item.name == q.name {
            if idx > 0 {
                prev = rows[idx - 1].name.clone();
            }
            if idx + 1 < rows.len() {
                next = rows[idx + 1].name.clone();
            }
            break;
        }
    }
    Ok(ok(vec![prev, next]))
}
