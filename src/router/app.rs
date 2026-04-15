use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

use super::{dashboard, public};

/// 构建应用总路由并注入中间件与状态
pub fn build_app(state: AppState) -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(public::routes())
        .merge(dashboard::routes(state.clone()))
        .layer(cors_layer)
        .with_state(state)
}
