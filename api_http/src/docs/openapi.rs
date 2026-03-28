use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::health_check::check::__path_health_check_docs;
use crate::docs::schemas::{
    ErrorResponse, FailResponse, HealthCheckSuccessResponse, RegisterUserSuccessResponse,
};
use crate::docs::users::register::__path_register_user_docs;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check_docs,
        register_user_docs
    ),
    components(
        schemas(
            FailResponse,
            ErrorResponse,
            HealthCheckSuccessResponse,
            RegisterUserSuccessResponse
        )
    ),
    tags(
        (name = "system", description = "System and health endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
}
