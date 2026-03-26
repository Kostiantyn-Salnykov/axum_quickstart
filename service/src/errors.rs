#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Internal error.")]
    Internal,

    #[error("Not found")]
    NotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation failed: {0}")]
    Validation(String),
}
