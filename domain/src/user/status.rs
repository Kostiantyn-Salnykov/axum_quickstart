use crate::errors::DomainError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Unconfirmed,
    Confirmed,
    ResetRequired,
    ForceChangePassword,
    ExternalProvider,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unconfirmed => "unconfirmed",
            Self::Confirmed => "confirmed",
            Self::ResetRequired => "reset_required",
            Self::ForceChangePassword => "force_change_password",
            Self::ExternalProvider => "external_provider",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Confirmed | Self::ExternalProvider)
    }

    pub fn can_login(&self) -> bool {
        !matches!(self, Self::Unconfirmed)
    }
}

impl FromStr for UserStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unconfirmed" => Ok(Self::Unconfirmed),
            "confirmed" => Ok(Self::Confirmed),
            "reset_required" => Ok(Self::ResetRequired),
            "force_change_password" => Ok(Self::ForceChangePassword),
            "external_provider" => Ok(Self::ExternalProvider),
            _ => Err(DomainError::UnknownUserStatus(s.to_owned())),
        }
    }
}
