#![allow(dead_code)]
#[utoipa::path(
    get,
    path = "/v1/health_check/",
    tag = "system",
    responses(
        (
            status = 200,
            description = "Health check completed successfully",
            body = crate::docs::schemas::HealthCheckSuccessResponse
        ),
        (
            status = 500,
            description = "Internal server error",
            body = crate::docs::schemas::ErrorResponse
        )
    )
)]
fn health_check_docs() {}
