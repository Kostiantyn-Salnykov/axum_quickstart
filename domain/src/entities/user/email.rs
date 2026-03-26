use crate::errors::DomainError;
use email_address::EmailAddress;
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
}
