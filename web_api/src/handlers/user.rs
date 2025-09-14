use axum::{
    extract::{Path, State},
    response::Json
};

use crate::entities::users::{UserCreatePayload, UserResponse};
use crate::entities::api_response::{ApiResponse, ApiError};
use crate::middleware::auth::AuthUser;


use crate::models::users::User;
use crate::models::follows::Follow;
use crate::db::DbPool;

pub async fn get_user_by_id(
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
    AuthUser { user_id: _ }: AuthUser,
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


pub async fn follow_user(
    AuthUser { user_id }: AuthUser, 
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let follower_id = user_id.parse::<u64>()
        .map_err(|_| ApiError::BadRequest("ID de usuario inválido".to_string()))?;

    let followed_id = id;

    if follower_id == followed_id {
        return Err(ApiError::BadRequest("No puedes seguirte a ti mismo".to_string()));
    }


    let already_following = Follow::exists(follower_id, followed_id, &pool).await
        .map_err(|_| ApiError::InternalError("Error al verificar seguimiento".to_string()))?;

    if already_following {
        return Err(ApiError::BadRequest("Ya sigues a este usuario".to_string()));
    }

    Follow::create(follower_id, followed_id, &pool).await
        .map_err(|_| ApiError::InternalError("Error al seguir usuario".to_string()))?;

    let response = ApiResponse::success_with_message((), "Usuario seguido exitosamente");
    Ok(Json(response))
}

pub async fn unfollow_user(
    AuthUser { user_id }: AuthUser, 
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let follower_id = user_id.parse::<u64>()
        .map_err(|_| ApiError::BadRequest("ID de usuario inválido".to_string()))?;

    let followed_id = id;

    if follower_id == followed_id {
        return Err(ApiError::BadRequest("No puedes dejar de seguirte a ti mismo".to_string()));
    }

    
    let is_following = Follow::exists(follower_id, followed_id, &pool).await
        .map_err(|_| ApiError::InternalError("Error al verificar seguimiento".to_string()))?;

    if !is_following {
        return Err(ApiError::BadRequest("No sigues a este usuario".to_string()));
    }

    Follow::delete(follower_id, followed_id, &pool).await
        .map_err(|_| ApiError::InternalError("Error al dejar de seguir usuario".to_string()))?;

    let response = ApiResponse::success_with_message((), "Usuario dejado de seguir exitosamente");
    Ok(Json(response))
}

pub async fn get_followers_count(
    Path(id): Path<u64>,
    State(pool): State<DbPool>,
) -> Result<Json<ApiResponse<u64>>, ApiError> {
    let count = Follow::get_follower_count(id, &pool).await
        .map_err(|_| ApiError::InternalError("Error al obtener el contador de seguidores".to_string()))?;

    let response = ApiResponse::success_with_message(count, "Contador de seguidores obtenido exitosamente");
    Ok(Json(response))
}