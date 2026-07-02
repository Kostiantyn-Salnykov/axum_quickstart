use crate::authorization::action::AuthorizationAction;
use crate::authorization::policy::AuthorizationPolicy;
use crate::authorization::resource::{AuthorizationResource, AuthorizationSubject};
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthorizationEnforcerPort: Send + Sync {
    async fn enforce(
        &self,
        subject: AuthorizationSubject,
        resource: AuthorizationResource,
        action: AuthorizationAction,
    ) -> Result<bool, ServiceError>;

    async fn add_policy(&self, policy: AuthorizationPolicy) -> Result<(), ServiceError>;

    async fn remove_policy(&self, policy: AuthorizationPolicy) -> Result<bool, ServiceError>;
}
