use std::env;

use axum::{extract::State, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::{db::DbPool, entities::{auth::{LoginPayload, LoginResponse}, claims::Claims, api_response::{ApiResponse, ApiError}}};
use crate::models::users::User;



pub async fn login(
    State(pool): State<DbPool>,
    Json(login_payload): Json<LoginPayload>
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let user = User::get_by_email(&login_payload.email, &pool).await
        .map_err(|_| ApiError::NotFound("Usuario no encontrado".to_string()))?;

    let password_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| ApiError::InternalError("Error al procesar el hash".to_string()))?;

    Argon2::default().verify_password(login_payload.password.as_bytes(), &password_hash)
        .map_err(|_| ApiError::Unauthorized("Contraseña incorrecta".to_string()))?;

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET")
        .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| ApiError::InternalError("Error al generar el token".to_string()))?;

    let response = ApiResponse::success_with_message(
        LoginResponse { token }, 
        "Login exitoso"
    );

    Ok(Json(response))
}