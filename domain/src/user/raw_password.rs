use crate::errors::DomainError;

#[derive(Debug, Clone)]
pub struct RawPassword(String);

impl RawPassword {
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

impl AsRef<str> for RawPassword {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_password_and_trims_whitespace() {
        let password = RawPassword::new("  Strong1!  ").unwrap();

        assert_eq!(password.as_str(), "Strong1!");
        assert_eq!(password.as_ref(), "Strong1!");
    }

    #[test]
    fn rejects_password_shorter_than_minimum_length() {
        let result = RawPassword::new("Aa1!");

        assert!(matches!(
            result,
            Err(DomainError::InvalidPassword(message))
            if message == "Password must be at least 8 characters long."
        ));
    }

    #[test]
    fn rejects_password_without_letters() {
        let result = RawPassword::new("1234567!");

        assert!(matches!(
            result,
            Err(DomainError::InvalidPassword(message))
            if message == "Password must contain letters."
        ));
    }

    #[test]
    fn rejects_password_without_uppercase_letter() {
        let result = RawPassword::new("password1!");

        assert!(matches!(
            result,
            Err(DomainError::InvalidPassword(message))
            if message == "Password must contain at least one uppercase letter."
        ));
    }

    #[test]
    fn rejects_password_without_number() {
        let result = RawPassword::new("Password!");

        assert!(matches!(
            result,
            Err(DomainError::InvalidPassword(message))
            if message == "Password must contain at least one number."
        ));
    }

    #[test]
    fn rejects_password_without_special_character() {
        let result = RawPassword::new("Password1");

        assert!(matches!(
            result,
            Err(DomainError::InvalidPassword(message))
            if message.contains("Password must contain at least one special character")
        ));
    }
}
