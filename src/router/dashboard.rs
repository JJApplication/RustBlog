use axum::{
    middleware as axum_middleware,
    routing::{post, put},
    Router,
};

use crate::{handlers, middleware::auth::jwt_auth_middleware, state::AppState};

/// 受保护路由（需要JWT）
pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/api/dashboard/post",
            post(handlers::dashboard_post::post)
                .put(handlers::dashboard_post::put)
                .delete(handlers::dashboard_post::delete),
        )
        .route(
            "/api/dashboard/tag",
            post(handlers::dashboard_tag::post)
                .put(handlers::dashboard_tag::put)
                .delete(handlers::dashboard_tag::delete),
        )
        .route(
            "/api/dashboard/comment",
            put(handlers::dashboard_comment::put).delete(handlers::dashboard_comment::delete),
        )
        .route(
            "/api/dashboard/message",
            put(handlers::dashboard_message::put).delete(handlers::dashboard_message::delete),
        )
        .layer(axum_middleware::from_fn_with_state(
            state,
            jwt_auth_middleware,
        ))
}
