use crate::authorization::policy::{AuthorizationPolicy, AuthorizationPolicyId};
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthorizationPolicyRepositoryPort: Send + Sync {
    async fn upsert(&self, policy: AuthorizationPolicy) -> Result<(), ServiceError>;

    async fn delete(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError>;

    async fn list(
        &self,
        policy_type: Option<&str>,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError>;

    async fn clear(&self, policy_type: Option<&str>) -> Result<u64, ServiceError>;
}
