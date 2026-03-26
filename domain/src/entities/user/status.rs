use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Unconfirmed,
    Confirmed,
    ResetRequired,
    ForceChangePassword,
    ExternalProvider,
}

impl UserStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Confirmed | Self::ExternalProvider)
    }

    pub fn can_login(&self) -> bool {
        !matches!(self, Self::Unconfirmed)
    }
}
