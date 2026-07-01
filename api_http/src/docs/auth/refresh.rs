#![allow(dead_code)]
use crate::auth::refresh::request::RefreshRequest;

#[utoipa::path(
    post,
    path = "/v1/auth/token/refresh/",
    tag = "auth",
    request_body(
        description = "Refresh token payload",
        content(
            (RefreshRequest = "application/json", examples(
                ("1" = (
                    summary = "Refresh access and refresh tokens",
                    value = json!({
                        "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
                    })
                )),
            )),
            (RefreshRequest = "application/msgpack")
        )
    ),
    responses(
        (
            status = 200,
            description = "Tokens refreshed successfully.",
            content(
                (crate::docs::schemas::AuthRefreshSuccessResponse = "application/json"),
                (crate::docs::schemas::AuthRefreshSuccessResponse = "application/msgpack")
            )
        ),
        (
            status = 400,
            description = "Bad request",
            content(
                (crate::docs::schemas::FailResponse = "application/json"),
                (crate::docs::schemas::FailResponse = "application/msgpack")
            )
        ),
        (
            status = 401,
            description = "Invalid or expired refresh token",
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
fn auth_refresh_docs() {}
