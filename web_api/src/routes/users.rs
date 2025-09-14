use axum::{
    routing::{get, post},
    Router,
};
use crate::db::DbPool;
use crate::handlers::user;

pub fn user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(user::create_user))
        .route("/:id", get(user::get_user_by_id))
        .route("/me", get(user::get_my_profile))
}
