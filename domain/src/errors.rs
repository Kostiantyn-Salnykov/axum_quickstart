use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid email format.")]
    InvalidEmail,

    #[error("{0}")]
    InvalidPassword(String),

    #[error("Invalid phone format.")]
    InvalidPhone,

    #[error("Operation isn't allowed for this status: {0}.")]
    InvalidStatusTransition(String),

    #[error("Unknown user status: {0}.")]
    UnknownUserStatus(String),

    #[error("Unknown auth provider: {0}.")]
    UnknownAuthProvider(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_all_error_messages() {
        assert_eq!(
            DomainError::InvalidEmail.to_string(),
            "Invalid email format."
        );
        assert_eq!(
            DomainError::InvalidPassword("bad password".to_string()).to_string(),
            "bad password"
        );
        assert_eq!(
            DomainError::InvalidPhone.to_string(),
            "Invalid phone format."
        );
        assert_eq!(
            DomainError::InvalidStatusTransition("archived".to_string()).to_string(),
            "Operation isn't allowed for this status: archived."
        );
        assert_eq!(
            DomainError::UnknownUserStatus("blocked".to_string()).to_string(),
            "Unknown user status: blocked."
        );
        assert_eq!(
            DomainError::UnknownAuthProvider("twitter".to_string()).to_string(),
            "Unknown auth provider: twitter."
        );
    }
}
