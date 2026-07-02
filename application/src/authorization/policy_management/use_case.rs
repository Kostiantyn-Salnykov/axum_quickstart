use crate::authorization::policy::{AuthorizationPolicy, AuthorizationPolicyId};
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait PolicyManagementUseCase: Send + Sync {
    async fn upsert_policy(&self, policy: AuthorizationPolicy) -> Result<(), ServiceError>;

    async fn delete_policy(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError>;

    async fn list_policies(
        &self,
        policy_type: Option<&str>,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError>;

    async fn clear_policies(&self, policy_type: Option<&str>) -> Result<u64, ServiceError>;
}
