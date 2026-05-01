#![allow(dead_code)]
use crate::auth::login::request::LoginRequest;

#[utoipa::path(
    post,
    path = "/v1/auth/login/",
    tag = "auth",
    request_body(
        content = LoginRequest,
        description = "User login payload",
        content_type = "application/json",
        examples(
            ("1" = (
                summary = "Login user",
                value = json!({
                    "email": "kostiantyn.salnykov@gmail.com",
                    "password": "Fake123password!",
                })
            )),
        )
    ),
    responses(
        (
            status = 200,
            description = "User logged in successfully.",
            body = crate::docs::schemas::AuthLoginSuccessResponse
        ),
        (
            status = 400,
            description = "Bad request",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 401,
            description = "Invalid credentials",
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
fn auth_login_docs() {}
