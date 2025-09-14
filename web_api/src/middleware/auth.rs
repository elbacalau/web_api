use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::entities::claims::Claims;
use crate::entities::api_response::ApiError;

pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(ApiError::Unauthorized("Token de autorización requerido".to_string()))?;

        let token = auth_header.strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized("Formato de token inválido".to_string()))?;

        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| ApiError::InternalError("JWT_SECRET no configurado".to_string()))?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| ApiError::Unauthorized("Token inválido o expirado".to_string()))?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}
