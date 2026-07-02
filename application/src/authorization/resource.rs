use std::collections::BTreeMap;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationSubject {
    pub user_id: Uuid,
    pub attributes: BTreeMap<String, String>,
}

impl AuthorizationSubject {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            attributes: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationResource {
    pub kind: String,
    pub resource_id: Uuid,
    pub attributes: BTreeMap<String, String>,
}

impl AuthorizationResource {
    pub fn new(kind: impl Into<String>, resource_id: Uuid) -> Self {
        Self {
            kind: kind.into(),
            resource_id,
            attributes: BTreeMap::new(),
        }
    }
}
