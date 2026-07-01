#![allow(dead_code)]
#[utoipa::path(
    get,
    path = "/v1/health_check/",
    tag = "system",
    responses(
        (
            status = 200,
            description = "Health check completed successfully",
            content(
                (crate::docs::schemas::HealthCheckSuccessResponse = "application/json"),
                (crate::docs::schemas::HealthCheckSuccessResponse = "application/msgpack")
            )
        ),
        (
            status = 500,
            description = "Internal server error",
            content(
                (crate::docs::schemas::ErrorResponse = "application/json"),
                (crate::docs::schemas::ErrorResponse = "application/msgpack")
            )
        )
    )
)]
fn health_check_docs() {}
