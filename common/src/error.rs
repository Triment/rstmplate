
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum CommonError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("jsonwebtoken error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("unauthorized access: {0}")]
    Unauthorized(String),
}

impl CommonError {
    pub fn message(&self) -> String {
        match self {
            CommonError::Database(e) => format!("Database error: {}", e),
            CommonError::InvalidEntity(_) => "Invalid entity provided".to_string(),
            CommonError::Anyhow(e) => format!("An error occurred: {}", e),
            CommonError::JwtError(e) => format!("JWT error: {}", e),
            CommonError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for CommonError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            CommonError::Database(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            CommonError::InvalidEntity(_) => axum::http::StatusCode::BAD_REQUEST,
            CommonError::Anyhow(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            CommonError::JwtError(_) => axum::http::StatusCode::UNAUTHORIZED,
            CommonError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
        };

        let error_response = ErrorResponse {
            error: self.to_string(),
            details: None, // Optionally include more details
        };

        (status, axum::Json(error_response)).into_response()
    }
}