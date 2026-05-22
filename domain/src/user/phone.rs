use crate::errors::DomainError;
use phonenumber::{Mode, parse};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_valid_phone_to_e164() {
        let phone = Phone::new("+1 (202) 555-0188").unwrap();

        assert_eq!(phone.as_str(), "+12025550188");
        assert_eq!(phone.as_ref(), "+12025550188");
        assert_eq!(phone.to_string(), "+12025550188");
    }

    #[test]
    fn rejects_invalid_phone() {
        let result = Phone::new("abc");

        assert!(matches!(result, Err(DomainError::InvalidPhone)));
    }
}
