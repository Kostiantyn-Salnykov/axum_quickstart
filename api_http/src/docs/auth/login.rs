#![allow(dead_code)]
use crate::auth::login::request::LoginRequest;

#[utoipa::path(
    post,
    path = "/v1/auth/login/",
    tag = "auth",
    request_body(
        description = "User login payload",
        content(
            (LoginRequest = "application/json", examples(
                ("1" = (
                    summary = "Login user",
                    value = json!({
                        "email": "kostiantyn.salnykov@gmail.com",
                        "password": "Fake123password!",
                    })
                )),
            )),
            (LoginRequest = "application/msgpack")
        )
    ),
    responses(
        (
            status = 200,
            description = "User logged in successfully.",
            content(
                (crate::docs::schemas::AuthLoginSuccessResponse = "application/json"),
                (crate::docs::schemas::AuthLoginSuccessResponse = "application/msgpack")
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
            description = "Invalid credentials",
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
fn auth_login_docs() {}
