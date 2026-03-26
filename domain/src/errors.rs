use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid email format.")]
    InvalidEmail,

    #[error("Invalid password.")]
    InvalidPassword,

    #[error("Operation isn't allowed for this status: {0:?}.")]
    InvalidStatusTransition(String),
}
