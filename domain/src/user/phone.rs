use crate::errors::DomainError;
use phonenumber::{Mode, country, parse};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phone(String);

impl Phone {
    pub fn new(value: &str) -> Result<Self, DomainError> {
        let parsed = parse(None, value).map_err(|_| DomainError::InvalidPhone)?;
        Ok(Self(parsed.format().mode(Mode::E164).to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
