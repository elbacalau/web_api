use serde::{Serialize, Deserialize};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::response::Response;
use axum::body::Body;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}


impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { 
            success: true, 
            data: Some(data), 
            error: None, 
            message: None 
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self { 
            success: true, 
            data: Some(data), 
            error: None, 
            message: Some(message.to_string()) 
        }
    }
}


#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized(String),
    InternalError(String),
    ValidationError(String),
    BadRequest(String),
}


impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let (status, error) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some(error),
            message: None,
        });
        
        (status, body).into_response()
    }
}