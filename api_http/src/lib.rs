use application::auth::login::use_case::LoginUseCase;
use application::auth::logout::use_case::LogoutUseCase;
use application::auth::refresh::use_case::RefreshUseCase;
use application::auth::register::use_case::RegisterUseCase;
use application::auth::token_manager_port::TokenManagerPort;
use application::system::health_check::use_case::HealthCheckUseCase;
use application::users::get::use_case::GetUserUseCase;
use axum::http::HeaderName;
use axum::middleware::from_fn_with_state;
use axum::{Router, http};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;

pub mod auth;
mod docs;
mod enums;
mod errors;
pub mod health_check;
pub(crate) mod middlewares;
mod responses;
pub mod state;
pub mod users;

use middlewares::MakeRequestUuid;
use state::AppState;

const REQUEST_ID_HEADER: &str = "x-request-id";
const API_VERSION: &str = "/v1";

pub struct ApiServices {
    pub system_health_check: Arc<dyn HealthCheckUseCase>,
    pub auth_register: Arc<dyn RegisterUseCase>,
    pub auth_login: Arc<dyn LoginUseCase>,
    pub auth_logout: Arc<dyn LogoutUseCase>,
    pub auth_refresh: Arc<dyn RefreshUseCase>,
    pub auth_token_manager: Arc<dyn TokenManagerPort>,
    pub users_get: Arc<dyn GetUserUseCase>,
}

impl From<ApiServices> for AppState {
    fn from(services: ApiServices) -> Self {
        Self {
            system: state::SystemState {
                health_check: services.system_health_check,
            },
            auth: state::AuthState {
                register: services.auth_register,
                login: services.auth_login,
                logout: services.auth_logout,
                refresh: services.auth_refresh,
                token_manager: services.auth_token_manager,
            },
            users: state::UsersState {
                get: services.users_get,
            },
        }
    }
}

pub fn create_router(services: ApiServices) -> Router {
    let state = AppState::from(services);
    let request_id_header = HeaderName::from_static(REQUEST_ID_HEADER);
    let protected_api_v1 = Router::new()
        .merge(auth::protected_router())
        .merge(users::router())
        .route_layer(from_fn_with_state(state.clone(), middlewares::require_auth));
    let api_v1 = Router::new()
        .merge(auth::public_router())
        .merge(health_check::router())
        .merge(protected_api_v1);

    Router::new()
        .merge(docs::router())
        .nest(API_VERSION, api_v1)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    request_id_header.clone(),
                    MakeRequestUuid,
                ))
                .layer({
                    let request_id_header = request_id_header.clone();

                    TraceLayer::new_for_http()
                        .make_span_with(move |request: &http::Request<_>| {
                            let request_id = request
                                .headers()
                                .get(&request_id_header)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("unknown");

                            tracing::debug_span!(
                                "http-request",
                                method = %request.method(),
                                path = %request.uri().path(),
                                request_id = %request_id,
                                status_code = tracing::field::Empty,
                            )
                        })
                        .on_response(
                            |response: &http::Response<_>,
                             latency: std::time::Duration,
                             span: &Span| {
                                span.record("status_code", response.status().as_u16());
                                tracing::debug!(latency_ms = latency.as_millis(), "response");
                            },
                        )
                        .on_failure(|error, latency: std::time::Duration, _span: &Span| {
                            tracing::error!(
                                error = %error,
                                latency_ms = latency.as_millis(),
                                "request failed"
                            );
                        })
                })
                .layer(PropagateRequestIdLayer::new(request_id_header)),
        )
        .with_state(state)
}
