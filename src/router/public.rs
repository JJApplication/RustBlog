use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, state::AppState};

/// 公开路由（无需JWT）
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/hello", get(handlers::hello::handle))
        .route("/api/admin/login", post(handlers::admin_login::handle))
        .route("/api/article/posts", get(handlers::article_posts::handle))
        .route("/api/article/post", get(handlers::article_post::handle))
        .route("/api/article/tags", get(handlers::article_tags::handle))
        .route("/api/article/tag", get(handlers::article_tag::handle))
        .route("/api/article/brother", get(handlers::article_brother::handle))
        .route("/api/article/search", get(handlers::article_search::handle))
        .route(
            "/api/article/comments",
            get(handlers::article_comments::get).post(handlers::article_comments::post),
        )
        .route(
            "/api/article/likes",
            get(handlers::article_likes::get).post(handlers::article_likes::post),
        )
        .route("/api/article/views", get(handlers::article_views::handle))
        .route("/api/article/archive", get(handlers::article_archive::handle))
        .route("/api/article/archives", get(handlers::article_archives::handle))
        .route(
            "/api/message",
            get(handlers::message::get).post(handlers::message::post),
        )
}
