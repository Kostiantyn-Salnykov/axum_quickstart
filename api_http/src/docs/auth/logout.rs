#![allow(dead_code)]

#[utoipa::path(
    post,
    path = "/v1/auth/logout/",
    tag = "auth",
    security(
        ("bearerAuth" = [])
    ),
    responses(
        (
            status = 200,
            description = "User logged out successfully.",
            content(
                (crate::docs::schemas::AuthLogoutSuccessResponse = "application/json"),
                (crate::docs::schemas::AuthLogoutSuccessResponse = "application/msgpack")
            )
        ),
        (
            status = 401,
            description = "Missing, invalid, or non-access bearer token",
            content(
                (crate::docs::schemas::FailResponse = "application/json", example = json!({
                    "status": "fail",
                    "code": 401,
                    "message": "Unauthorized"
                })),
                (crate::docs::schemas::FailResponse = "application/msgpack")
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
fn auth_logout_docs() {}
