use crate::authorization::action::AuthorizationAction;
use crate::authorization::resource::{AuthorizationResource, AuthorizationSubject};
use crate::authorization::result::AuthorizationResult;
use crate::errors::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthorizationUseCase: Send + Sync {
    async fn authorize(
        &self,
        subject: AuthorizationSubject,
        resource: AuthorizationResource,
        action: AuthorizationAction,
    ) -> Result<AuthorizationResult, ServiceError>;
}
