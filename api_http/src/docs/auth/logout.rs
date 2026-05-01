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
            body = crate::docs::schemas::AuthLogoutSuccessResponse
        ),
        (
            status = 401,
            description = "Missing or invalid bearer token",
            body = crate::docs::schemas::FailResponse,
            example = json!({
                "status": "fail",
                "code": 401,
                "message": "Unauthorized"
            })
        ),
        (
            status = 500,
            description = "Internal server error",
            body = crate::docs::schemas::ErrorResponse
        )
    )
)]
fn auth_logout_docs() {}
