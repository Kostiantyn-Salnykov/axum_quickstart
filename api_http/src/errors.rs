use application::errors::ServiceError;
use axum::extract::rejection::JsonRejection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn from_json_rejection(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::JsonDataError(err) => Self::Validation(err.body_text()),
            JsonRejection::JsonSyntaxError(err) => Self::BadRequest(err.body_text()),
            JsonRejection::MissingJsonContentType(_) => {
                Self::BadRequest("Missing `content-type: application/json` header.".to_string())
            }
            other => Self::BadRequest(other.body_text()),
        }
    }
}

impl From<ServiceError> for AppError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::Validation(message) => Self::Validation(message),
            ServiceError::Conflict(message) => Self::Conflict(message),
            ServiceError::NotFound => Self::NotFound("Resource not found.".to_string()),
            ServiceError::InvalidCredentials => Self::Unauthorized,
            ServiceError::Internal { source } => Self::Internal(source),
        }
    }
}
