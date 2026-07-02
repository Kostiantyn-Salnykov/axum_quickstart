use crate::rate_limit::policy::RateLimitInfo;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Internal error.")]
    Internal {
        #[source]
        source: anyhow::Error,
    },

    #[error("Not found")]
    NotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited: {message}")]
    RateLimited {
        info: RateLimitInfo,
        message: String,
    },

    #[error("Validation failed: {0}")]
    Validation(String),
}

impl ServiceError {
    pub fn internal(error: impl Into<anyhow::Error>) -> Self {
        Self::Internal {
            source: error.into(),
        }
    }
}
