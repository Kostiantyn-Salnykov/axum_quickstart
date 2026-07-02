use crate::authorization::action::AuthorizationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationAccessRole {
    Owner,
    Reader,
    Writer,
}

impl AuthorizationAccessRole {
    pub fn allowed_actions(self) -> &'static [AuthorizationAction] {
        match self {
            Self::Owner => &[
                AuthorizationAction::Read,
                AuthorizationAction::Update,
                AuthorizationAction::Delete,
            ],
            Self::Writer => &[AuthorizationAction::Read, AuthorizationAction::Update],
            Self::Reader => &[AuthorizationAction::Read],
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Reader => "reader",
            Self::Writer => "writer",
        }
    }
}
