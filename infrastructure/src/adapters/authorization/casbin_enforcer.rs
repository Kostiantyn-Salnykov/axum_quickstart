use std::path::{Path, PathBuf};
use std::sync::Arc;

use application::authorization::action::AuthorizationAction;
use application::authorization::enforcer_port::AuthorizationEnforcerPort;
use application::authorization::policy::AuthorizationPolicy;
use application::authorization::resource::{AuthorizationResource, AuthorizationSubject};
use application::errors::ServiceError;
use async_trait::async_trait;
use casbin::MgmtApi;
use casbin::prelude::{CoreApi, DefaultModel, Enforcer};
use tokio::sync::RwLock;

use crate::adapters::redis::client::RedisClient;
use crate::adapters::redis::policy_store::RedisPolicyStoreAdapter;

#[derive(Clone)]
pub struct CasbinAuthorizationEnforcerAdapter {
    enforcer: Arc<RwLock<Enforcer>>,
}

impl CasbinAuthorizationEnforcerAdapter {
    pub async fn new(
        model_path: impl AsRef<Path>,
        redis_client: RedisClient,
    ) -> std::result::Result<Self, ServiceError> {
        let model = DefaultModel::from_file(model_path.as_ref())
            .await
            .map_err(ServiceError::internal)?;
        let adapter = RedisPolicyStoreAdapter::new(redis_client);
        let enforcer = Enforcer::new(model, adapter)
            .await
            .map_err(ServiceError::internal)?;

        Ok(Self {
            enforcer: Arc::new(RwLock::new(enforcer)),
        })
    }

    pub async fn new_from_workspace(
        redis_client: RedisClient,
    ) -> std::result::Result<Self, ServiceError> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let model_path = manifest_dir.join("casbin").join("model.conf");
        Self::new(model_path, redis_client).await
    }
}

#[async_trait]
impl AuthorizationEnforcerPort for CasbinAuthorizationEnforcerAdapter {
    async fn enforce(
        &self,
        subject: AuthorizationSubject,
        resource: AuthorizationResource,
        action: AuthorizationAction,
    ) -> std::result::Result<bool, ServiceError> {
        let enforcer = self.enforcer.read().await;
        let allowed = enforcer
            .enforce((
                subject.user_id.to_string(),
                resource_key(&resource.kind, resource.resource_id),
                action_as_str(action).to_string(),
            ))
            .map_err(ServiceError::internal)?;

        Ok(allowed)
    }

    async fn add_policy(
        &self,
        policy: AuthorizationPolicy,
    ) -> std::result::Result<(), ServiceError> {
        let mut enforcer = self.enforcer.write().await;
        let params: Vec<String> = policy_to_rule(&policy);
        enforcer
            .add_policy(params)
            .await
            .map_err(ServiceError::internal)?;
        Ok(())
    }

    async fn remove_policy(
        &self,
        policy: AuthorizationPolicy,
    ) -> std::result::Result<bool, ServiceError> {
        let mut enforcer = self.enforcer.write().await;
        let params: Vec<String> = policy_to_rule(&policy);
        let removed = enforcer
            .remove_policy(params)
            .await
            .map_err(ServiceError::internal)?;
        Ok(removed)
    }
}

fn policy_to_rule(policy: &AuthorizationPolicy) -> Vec<String> {
    vec![
        policy.subject.clone(),
        policy.object.clone(),
        policy.action.clone(),
        effect_as_str(policy.effect).to_string(),
    ]
}

fn effect_as_str(effect: application::authorization::AuthorizationEffect) -> &'static str {
    match effect {
        application::authorization::AuthorizationEffect::Allow => "allow",
        application::authorization::AuthorizationEffect::Deny => "deny",
    }
}

fn action_as_str(action: AuthorizationAction) -> &'static str {
    match action {
        AuthorizationAction::Read => "read",
        AuthorizationAction::Create => "create",
        AuthorizationAction::Update => "update",
        AuthorizationAction::Delete => "delete",
        AuthorizationAction::Manage => "manage",
    }
}

fn resource_key(resource_kind: &str, resource_id: uuid::Uuid) -> String {
    format!("{resource_kind}:{resource_id}")
}
