use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, state::AppState};

/// 公开路由（无需JWT）
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/hello", get(handlers::hello::handle))
        .route("/auth", get(handlers::auth::check))
        .route("/admin/login", post(handlers::admin_login::handle))
        .route("/article/posts", get(handlers::article_posts::handle))
        .route("/article/post", get(handlers::article_post::handle))
        .route("/article/tags", get(handlers::article_tags::handle))
        .route("/article/tag", get(handlers::article_tag::handle))
        .route("/article/brother", get(handlers::article_brother::handle))
        .route("/article/search", get(handlers::article_search::handle))
        .route(
            "/article/comments",
            get(handlers::article_comments::get).post(handlers::article_comments::post),
        )
        .route(
            "/article/likes",
            get(handlers::article_likes::get).post(handlers::article_likes::post),
        )
        .route("/article/views", get(handlers::article_views::handle))
        .route("/article/archive", get(handlers::article_archive::handle))
        .route("/article/archives", get(handlers::article_archives::handle))
        .route("/zhuanlan", get(handlers::zhuanlan::list))
        .route("/zhuanlan/{link}", get(handlers::zhuanlan::detail))
        .route("/statistic/routines", get(handlers::statistic::routines))
        .route("/statistic/mem", get(handlers::statistic::mem))
        .route("/statistic/views", get(handlers::statistic::views))
        .route("/statistic/daily", get(handlers::statistic::daily))
        .route(
            "/message",
            get(handlers::message::get).post(handlers::message::post),
        )
}
