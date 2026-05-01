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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_provider() {
        let provider = AuthProvider::from_str("github").unwrap();

        assert_eq!(provider, AuthProvider::GitHub);
        assert_eq!(provider.as_str(), "github");
        assert_eq!(provider.to_string(), "github");
    }

    #[test]
    fn exposes_all_provider_labels() {
        assert_eq!(AuthProvider::Google.as_str(), "google");
        assert_eq!(AuthProvider::Meta.as_str(), "meta");
        assert_eq!(AuthProvider::GitHub.as_str(), "github");
    }

    #[test]
    fn rejects_unknown_provider() {
        let result = AuthProvider::from_str("twitter");

        assert!(matches!(
            result,
            Err(DomainError::UnknownAuthProvider(value))
            if value == "twitter"
        ));
    }
}
