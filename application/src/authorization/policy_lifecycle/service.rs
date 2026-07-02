use std::sync::Arc;

use crate::authorization::access_role::AuthorizationAccessRole;
use crate::authorization::action::AuthorizationAction;
use crate::authorization::enforcer_port::AuthorizationEnforcerPort;
use crate::authorization::policy::{
    AuthorizationEffect, AuthorizationPolicy, AuthorizationPolicyId,
};
use crate::authorization::policy_lifecycle::use_case::PolicyLifecycleUseCase;
use crate::errors::ServiceError;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct PolicyLifecycleService {
    enforcer: Arc<dyn AuthorizationEnforcerPort>,
}

impl PolicyLifecycleService {
    pub fn new(enforcer: Arc<dyn AuthorizationEnforcerPort>) -> Self {
        Self { enforcer }
    }
}

#[async_trait]
impl PolicyLifecycleUseCase for PolicyLifecycleService {
    async fn grant_owner(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError> {
        self.grant_access(
            resource_kind,
            resource_id,
            subject_id,
            AuthorizationAccessRole::Owner,
        )
        .await
    }

    async fn grant_access(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
        role: AuthorizationAccessRole,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError> {
        let object = resource_key(resource_kind, resource_id);
        let subject = subject_id.to_string();
        let policies = role
            .allowed_actions()
            .iter()
            .map(|action| policy_for(&subject, &object, *action))
            .collect::<Vec<_>>();

        for policy in &policies {
            self.enforcer.add_policy(policy.clone()).await?;
        }

        Ok(policies)
    }

    async fn revoke_access(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
        role: AuthorizationAccessRole,
    ) -> Result<u64, ServiceError> {
        let object = resource_key(resource_kind, resource_id);
        let subject = subject_id.to_string();
        let mut deleted = 0_u64;

        for action in role.allowed_actions() {
            let policy = policy_for(&subject, &object, *action);
            if self.enforcer.remove_policy(policy).await? {
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    async fn revoke_policy(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError> {
        let Some(policy) = policy_from_id(&policy_id) else {
            return Ok(false);
        };
        self.enforcer.remove_policy(policy).await
    }
}

fn policy_for(subject: &str, object: &str, action: AuthorizationAction) -> AuthorizationPolicy {
    AuthorizationPolicy::new(
        "p",
        subject,
        object,
        action.as_str(),
        AuthorizationEffect::Allow,
    )
}

fn policy_from_id(policy_id: &str) -> Option<AuthorizationPolicy> {
    let parts = policy_id.rsplitn(2, ':').collect::<Vec<_>>();
    let effect = match parts.first().copied()? {
        "allow" => AuthorizationEffect::Allow,
        "deny" => AuthorizationEffect::Deny,
        _ => return None,
    };

    let head = parts.get(1)?.rsplitn(4, ':').collect::<Vec<_>>();
    if head.len() != 4 {
        return None;
    }

    Some(AuthorizationPolicy {
        id: policy_id.to_string(),
        policy_type: head[3].to_string(),
        subject: head[2].to_string(),
        object: head[1].to_string(),
        action: head[0].to_string(),
        effect,
    })
}

fn resource_key(resource_kind: &str, resource_id: Uuid) -> String {
    format!("{resource_kind}:{resource_id}")
}

trait AuthorizationActionExt {
    fn as_str(&self) -> &'static str;
}

impl AuthorizationActionExt for AuthorizationAction {
    fn as_str(&self) -> &'static str {
        match self {
            AuthorizationAction::Read => "read",
            AuthorizationAction::Create => "create",
            AuthorizationAction::Update => "update",
            AuthorizationAction::Delete => "delete",
            AuthorizationAction::Manage => "manage",
        }
    }
}
