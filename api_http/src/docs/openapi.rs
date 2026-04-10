use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::auth::register::__path_auth_register_docs;
use crate::docs::health_check::check::__path_health_check_docs;
use crate::docs::schemas::{
    AuthRegisterSuccessResponse, ErrorResponse, FailResponse, HealthCheckSuccessResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check_docs,
        auth_register_docs
    ),
    components(
        schemas(
            FailResponse,
            ErrorResponse,
            HealthCheckSuccessResponse,
            AuthRegisterSuccessResponse
        )
    ),
    tags(
        (name = "system", description = "System and health endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}
