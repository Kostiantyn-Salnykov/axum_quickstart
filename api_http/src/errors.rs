use application::errors::ServiceError;
use application::rate_limit::policy::RateLimitInfo;
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

    #[error("Rate limited: {message}")]
    RateLimited {
        info: RateLimitInfo,
        message: String,
    },

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

    pub fn from_content_body_rejection(rejection: crate::content::ContentBodyRejection) -> Self {
        rejection.into_app_error()
    }
}

impl From<ServiceError> for AppError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::Validation(message) => Self::Validation(message),
            ServiceError::Conflict(message) => Self::Conflict(message),
            ServiceError::RateLimited { info, message } => Self::RateLimited { info, message },
            ServiceError::NotFound => Self::NotFound("Resource not found.".to_string()),
            ServiceError::InvalidCredentials => Self::Unauthorized,
            ServiceError::Internal { source } => Self::Internal(source),
        }
    }
}
