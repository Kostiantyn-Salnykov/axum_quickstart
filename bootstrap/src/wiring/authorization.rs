use std::sync::Arc;

use api_http::state::AuthorizationState;
use application::authorization::AuthorizationEnforcerPort;
use application::authorization::policy_lifecycle::service::PolicyLifecycleService;
use application::authorization::policy_lifecycle::use_case::PolicyLifecycleUseCase;
use application::authorization::service::AuthorizationService;
use application::authorization::use_case::AuthorizationUseCase;
use infrastructure::adapters;
use infrastructure::adapters::redis::client::RedisClient;

pub async fn build_authorization_services(redis_client: RedisClient) -> AuthorizationState {
    let enforcer: Arc<dyn AuthorizationEnforcerPort> = Arc::new(
        adapters::authorization::casbin_enforcer::CasbinAuthorizationEnforcerAdapter::new_from_workspace(
            redis_client,
        )
            .await
            .expect("failed to initialize casbin enforcer"),
    );

    let authorize: Arc<dyn AuthorizationUseCase> =
        Arc::new(AuthorizationService::new(enforcer.clone()));
    let lifecycle: Arc<dyn PolicyLifecycleUseCase> =
        Arc::new(PolicyLifecycleService::new(enforcer));

    AuthorizationState {
        authorize,
        lifecycle,
    }
}
