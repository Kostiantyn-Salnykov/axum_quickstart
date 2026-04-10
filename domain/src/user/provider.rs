use crate::errors::DomainError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthProvider {
    Google,
    Meta,
    GitHub,
}

impl AuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "google",
            Self::Meta => "meta",
            Self::GitHub => "github",
        }
    }
}

impl FromStr for AuthProvider {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "google" => Ok(Self::Google),
            "meta" => Ok(Self::Meta),
            "github" => Ok(Self::GitHub),
            _ => Err(DomainError::UnknownAuthProvider(s.to_owned())),
        }
    }
}

impl Display for AuthProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
