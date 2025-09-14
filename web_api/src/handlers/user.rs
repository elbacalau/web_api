use axum::{
    extract::{Path, State},
    response::Json
};

use crate::entities::users::{UserCreatePayload, UserResponse};
use crate::entities::api_response::{ApiResponse, ApiError};
use crate::middleware::auth::AuthUser;


use crate::models::users::User;
use crate::db::DbPool;

pub async fn get_user_by_id(
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let user = User::get_by_id(id, &pool).await
        .map_err(|_| ApiError::NotFound("Usuario no encontrado".to_string()))?;

    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    let response = ApiResponse::success(user_response);
    Ok(Json(response))
}


pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<UserCreatePayload>,
) -> Result<Json<ApiResponse<User>>, ApiError> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| ApiError::InternalError("Error al hashear la contraseña".to_string()))?
        .to_string();
    
    let user = User {
        id: 0,
        username: payload.username,
        email: payload.email,
        password_hash,
        created_at: None,
        updated_at: None,
    };
    
    let user = user.create(&pool).await
        .map_err(|_| ApiError::InternalError("Error al crear el usuario".to_string()))?;
    
    let response = ApiResponse::success_with_message(user, "Usuario creado exitosamente");
    Ok(Json(response))
}

pub async fn get_my_profile(
    State(pool): State<DbPool>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let user_id = user_id.parse::<u64>()
        .map_err(|_| ApiError::BadRequest("ID de usuario inválido".to_string()))?;

    let user = User::get_by_id(user_id, &pool).await
        .map_err(|_| ApiError::NotFound("Usuario no encontrado".to_string()))?;

    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    let response = ApiResponse::success_with_message(user_response, "Perfil obtenido exitosamente");
    Ok(Json(response))
}
