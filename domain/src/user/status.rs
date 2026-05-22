use crate::errors::DomainError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Unconfirmed,
    Confirmed,
    ForceChangePassword,
    WaitingForDeletion,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unconfirmed => "unconfirmed",
            Self::Confirmed => "confirmed",
            Self::ForceChangePassword => "force_change_password",
            Self::WaitingForDeletion => "waiting_for_deletion",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Confirmed | Self::ForceChangePassword)
    }

    pub fn can_login(&self) -> bool {
        matches!(self, Self::Confirmed | Self::ForceChangePassword)
    }
}

impl FromStr for UserStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unconfirmed" => Ok(Self::Unconfirmed),
            "confirmed" => Ok(Self::Confirmed),
            "force_change_password" => Ok(Self::ForceChangePassword),
            "waiting_for_deletion" => Ok(Self::WaitingForDeletion),
            _ => Err(DomainError::UnknownUserStatus(s.to_owned())),
        }
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_status_and_exposes_capabilities() {
        let status = UserStatus::from_str("force_change_password").unwrap();

        assert_eq!(status, UserStatus::ForceChangePassword);
        assert_eq!(status.as_str(), "force_change_password");
        assert!(status.is_active());
        assert!(status.can_login());
        assert_eq!(status.to_string(), "force_change_password");
    }

    #[test]
    fn exposes_all_status_labels() {
        assert_eq!(UserStatus::Unconfirmed.as_str(), "unconfirmed");
        assert_eq!(UserStatus::Confirmed.as_str(), "confirmed");
        assert_eq!(
            UserStatus::ForceChangePassword.as_str(),
            "force_change_password"
        );
        assert_eq!(
            UserStatus::WaitingForDeletion.as_str(),
            "waiting_for_deletion"
        );
    }

    #[test]
    fn inactive_status_cannot_login() {
        assert!(!UserStatus::Unconfirmed.is_active());
        assert!(!UserStatus::Unconfirmed.can_login());
        assert!(!UserStatus::WaitingForDeletion.is_active());
        assert!(!UserStatus::WaitingForDeletion.can_login());
        assert!(UserStatus::Confirmed.is_active());
        assert!(UserStatus::Confirmed.can_login());
    }

    #[test]
    fn rejects_unknown_status() {
        let result = UserStatus::from_str("blocked");

        assert!(matches!(
            result,
            Err(DomainError::UnknownUserStatus(value))
            if value == "blocked"
        ));
    }
}
