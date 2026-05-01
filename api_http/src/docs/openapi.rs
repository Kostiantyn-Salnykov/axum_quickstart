use axum::Router;
use utoipa::OpenApi;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::auth::login::__path_auth_login_docs;
use crate::docs::auth::logout::__path_auth_logout_docs;
use crate::docs::auth::refresh::__path_auth_refresh_docs;
use crate::docs::auth::register::__path_auth_register_docs;
use crate::docs::health_check::check::__path_health_check_docs;
use crate::docs::schemas::{
    AuthLoginSuccessResponse, AuthLogoutSuccessResponse, AuthRefreshSuccessResponse,
    AuthRegisterSuccessResponse, ErrorResponse, FailResponse, HealthCheckSuccessResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check_docs,
        auth_register_docs,
        auth_login_docs,
        auth_refresh_docs,
        auth_logout_docs,
    ),
    components(
        schemas(
            FailResponse,
            ErrorResponse,
            HealthCheckSuccessResponse,
            AuthRegisterSuccessResponse,
            AuthLoginSuccessResponse,
            AuthRefreshSuccessResponse,
            AuthLogoutSuccessResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "system", description = "System and health endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}
