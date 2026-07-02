use serde::{Deserialize, Serialize};

pub type AuthorizationPolicyId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizationEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    pub id: AuthorizationPolicyId,
    pub policy_type: String,
    pub subject: String,
    pub object: String,
    pub action: String,
    pub effect: AuthorizationEffect,
}

impl AuthorizationPolicy {
    pub fn new(
        policy_type: impl Into<String>,
        subject: impl Into<String>,
        object: impl Into<String>,
        action: impl Into<String>,
        effect: AuthorizationEffect,
    ) -> Self {
        let policy_type = policy_type.into();
        let subject = subject.into();
        let object = object.into();
        let action = action.into();
        let id = Self::make_id(&policy_type, &subject, &object, &action, effect);

        Self {
            id,
            policy_type,
            subject,
            object,
            action,
            effect,
        }
    }

    pub fn make_id(
        policy_type: &str,
        subject: &str,
        object: &str,
        action: &str,
        effect: AuthorizationEffect,
    ) -> AuthorizationPolicyId {
        format!(
            "{policy_type}:{subject}:{object}:{action}:{}",
            match effect {
                AuthorizationEffect::Allow => "allow",
                AuthorizationEffect::Deny => "deny",
            }
        )
    }
}
