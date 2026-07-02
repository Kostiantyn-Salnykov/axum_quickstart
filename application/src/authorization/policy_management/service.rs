use std::sync::Arc;

use crate::authorization::policy::{AuthorizationPolicy, AuthorizationPolicyId};
use crate::authorization::policy_management::use_case::PolicyManagementUseCase;
use crate::authorization::repository_port::AuthorizationPolicyRepositoryPort;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PolicyManagementService {
    policies: Arc<dyn AuthorizationPolicyRepositoryPort>,
}

impl PolicyManagementService {
    pub fn new(policies: Arc<dyn AuthorizationPolicyRepositoryPort>) -> Self {
        Self { policies }
    }
}

#[async_trait]
impl PolicyManagementUseCase for PolicyManagementService {
    async fn upsert_policy(&self, policy: AuthorizationPolicy) -> Result<(), ServiceError> {
        self.policies.upsert(policy).await
    }

    async fn delete_policy(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError> {
        self.policies.delete(policy_id).await
    }

    async fn list_policies(
        &self,
        policy_type: Option<&str>,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError> {
        self.policies.list(policy_type).await
    }

    async fn clear_policies(&self, policy_type: Option<&str>) -> Result<u64, ServiceError> {
        self.policies.clear(policy_type).await
    }
}
