use crate::authorization::access_role::AuthorizationAccessRole;
use crate::authorization::policy::{AuthorizationPolicy, AuthorizationPolicyId};
use crate::errors::ServiceError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PolicyLifecycleUseCase: Send + Sync {
    async fn grant_owner(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError>;

    async fn grant_access(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
        role: AuthorizationAccessRole,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError>;

    async fn revoke_access(
        &self,
        resource_kind: &str,
        resource_id: Uuid,
        subject_id: Uuid,
        role: AuthorizationAccessRole,
    ) -> Result<u64, ServiceError>;

    async fn revoke_policy(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError>;
}
