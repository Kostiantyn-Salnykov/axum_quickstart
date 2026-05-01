#![allow(dead_code)]
use crate::auth::refresh::request::RefreshRequest;

#[utoipa::path(
    post,
    path = "/v1/auth/token/refresh/",
    tag = "auth",
    request_body(
        content = RefreshRequest,
        description = "Refresh token payload",
        content_type = "application/json",
        examples(
            ("1" = (
                summary = "Refresh access and refresh tokens",
                value = json!({
                    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
                })
            )),
        )
    ),
    responses(
        (
            status = 200,
            description = "Tokens refreshed successfully.",
            body = crate::docs::schemas::AuthRefreshSuccessResponse
        ),
        (
            status = 400,
            description = "Bad request",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 401,
            description = "Invalid or expired refresh token",
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
fn auth_refresh_docs() {}
