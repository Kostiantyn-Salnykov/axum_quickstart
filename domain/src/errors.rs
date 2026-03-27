use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid email format.")]
    InvalidEmail,

    #[error("Invalid password.")]
    InvalidPassword,

    #[error("Operation isn't allowed for this status: {0}.")]
    InvalidStatusTransition(String),

    #[error("Unknown user status: {0}.")]
    UnknownUserStatus(String),

    #[error("Unknown auth provider: {0}.")]
    UnknownAuthProvider(String),
}
