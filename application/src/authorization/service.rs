use std::sync::Arc;

use crate::authorization::action::AuthorizationAction;
use crate::authorization::enforcer_port::AuthorizationEnforcerPort;
use crate::authorization::resource::{AuthorizationResource, AuthorizationSubject};
use crate::authorization::result::AuthorizationResult;
use crate::authorization::use_case::AuthorizationUseCase;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct AuthorizationService {
    authorization: Arc<dyn AuthorizationEnforcerPort>,
}

impl AuthorizationService {
    pub fn new(authorization: Arc<dyn AuthorizationEnforcerPort>) -> Self {
        Self { authorization }
    }
}

#[async_trait]
impl AuthorizationUseCase for AuthorizationService {
    async fn authorize(
        &self,
        subject: AuthorizationSubject,
        resource: AuthorizationResource,
        action: AuthorizationAction,
    ) -> Result<AuthorizationResult, ServiceError> {
        let allowed = self
            .authorization
            .enforce(subject, resource, action)
            .await?;

        Ok(AuthorizationResult { allowed })
    }
}
