use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};

use crate::entities::users::{UserCreatePayload, UserResponse};


use crate::models::users::User;
use crate::db::DbPool;

pub async fn get_user_by_id(
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
) -> Result<Json<UserResponse>, StatusCode> {
    match User::get_by_id(id, &pool).await {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}


pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<UserCreatePayload>,
) -> Result<Json<User>, StatusCode> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    
    let user = User {
        id: 0,
        username: payload.username,
        email: payload.email,
        password_hash,
        created_at: None,
        updated_at: None,
    };
    
    let user = user.create(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}
