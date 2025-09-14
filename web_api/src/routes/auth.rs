use axum::{
    routing::post,
    Router,
};

use crate::{db::DbPool, handlers::auth};

pub fn auth_routes() -> Router<DbPool> {
    Router::new()
        .route("/login", post(auth::login))
}