//Error handler file for quote server

use axum::{/*http::StatusCode,*/response::{IntoResponse, /*Response*/}};
use thiserror::Error;
use askama::Error as AskamaError;

/// app-level error type
#[derive(Error, Debug)]
pub enum AppError {
    
    #[error("Not found")]
    NotFound,

    #[error("database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("template rendering error: {0}")]
    TemplateError(#[from] AskamaError),

    /// io errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON errors
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error), 
}

impl IntoResponse for AppError {
    // into  basic HTTP response 
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => {
                (
                    axum::http::StatusCode::NOT_FOUND,
                    "404 Not Found".to_string(),
                )
                    .into_response()
            }
            err => {
                // other error, return 500 with debug msg.
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal error: {}", err),
                )
                    .into_response()
            }
        }
    }
}
