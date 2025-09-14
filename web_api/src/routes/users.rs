use axum::{
    routing::{get, post, delete},
    Router,
};
use crate::db::DbPool;
use crate::handlers::user;

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(user::create_user))
        .route("/:id", get(user::get_user_by_id))
        .route("/me", get(user::get_my_profile))
        .route("/:id/follow", post(user::follow_user))
        .route("/:id/unfollow", delete(user::unfollow_user))
        .route("/:id/followers/count", get(user::get_followers_count))
}
