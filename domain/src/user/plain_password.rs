use crate::errors::DomainError;

#[derive(Debug, Clone)]
pub struct PlainPassword(String);

impl PlainPassword {
    const MIN_LENGTH: usize = 8;
    const SPECIAL_CHARS: &'static str = "!@#$%^&*()_+-=[]{}|;':\",.<>?/ ";

    pub fn new(value: &str) -> Result<Self, DomainError> {
        let value = value.trim();

        if value.chars().count() < Self::MIN_LENGTH {
            return Err(DomainError::InvalidPassword(
                "Password must be at least 8 characters long.".to_string(),
            ));
        }

        if !value.chars().any(|ch| ch.is_ascii_alphabetic()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain letters.".to_string(),
            ));
        }

        if !value.chars().any(|ch| ch.is_ascii_uppercase()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one uppercase letter.".to_string(),
            ));
        }

        if !value.chars().any(|ch| ch.is_ascii_digit()) {
            return Err(DomainError::InvalidPassword(
                "Password must contain at least one number.".to_string(),
            ));
        }

        if !value.chars().any(|ch| Self::SPECIAL_CHARS.contains(ch)) {
            return Err(DomainError::InvalidPassword(format!(
                "Password must contain at least one special character from: {}.",
                Self::SPECIAL_CHARS
            )));
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PlainPassword {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
