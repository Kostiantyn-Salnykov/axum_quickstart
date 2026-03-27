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

    pub fn to_owned(&self) -> String {
        self.as_str().to_owned()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Email {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
