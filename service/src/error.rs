use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Status code {0}")]
    Status(StatusCode),
    #[error("Date parse code {0}")]
    DateInputError(jiff::Error),
    #[error("Date parse code {0}")]
    DateError(#[from] jiff::Error),
    #[error("DB error {0}")]
    DBError(#[from] duckdb::Error),
    #[error("Serde error {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!(message = "app error", error = %self);

        match self {
            AppError::Status(code) => code.into_response(),
            AppError::DateInputError(error) => {
                (StatusCode::BAD_REQUEST, error.to_string()).into_response()
            }
            AppError::DateError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
            AppError::DBError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
            AppError::SerdeError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        }
    }
}
