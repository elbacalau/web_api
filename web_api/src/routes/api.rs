use axum::Router;
use crate::routes::users;
use crate::db::DbPool;

pub fn create_routes() -> Router<DbPool> {
    Router::new()
        .nest("/users", users::user_routes())
}