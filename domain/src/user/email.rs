use crate::errors::DomainError;
use email_address::EmailAddress;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(EmailAddress);

impl Email {
    pub fn new(raw: &str) -> Result<Self, DomainError> {
        EmailAddress::from_str(raw.trim())
            .map(Email)
            .map_err(|_| DomainError::InvalidEmail)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn to_owned(&self) -> String {
        self.as_str().to_owned()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Email {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_email_and_trims_whitespace() {
        let email = Email::new("  user@example.com  ").unwrap();

        assert_eq!(email.as_str(), "user@example.com");
        assert_eq!(email.as_ref(), "user@example.com");
        assert_eq!(email.to_string(), "user@example.com");
        assert_eq!(email.to_owned(), "user@example.com");
    }

    #[test]
    fn parses_from_str() {
        let email = "user@example.com".parse::<Email>().unwrap();

        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn rejects_invalid_email() {
        let result = Email::new("not-an-email");

        assert!(matches!(result, Err(DomainError::InvalidEmail)));
    }
}
