use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum CommonError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl CommonError {
    pub fn message(&self) -> String {
        match self {
            CommonError::Database(e) => format!("Database error: {}", e),
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
        };

        let error_response = ErrorResponse {
            error: self.to_string(),
            details: None, // Optionally include more details
        };

        (status, axum::Json(error_response)).into_response()
    }
}