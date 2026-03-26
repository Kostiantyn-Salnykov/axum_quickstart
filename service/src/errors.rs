#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Not found")]
    NotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,
}
