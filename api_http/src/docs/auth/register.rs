#![allow(dead_code)]
use crate::auth::register::request::RegisterRequest;

#[utoipa::path(
    post,
    path = "/v1/auth/register/",
    tag = "auth",
    request_body(
        description = "User registration payload",
        content(
            (RegisterRequest = "application/json", examples(
                ("1" = (
                    summary = "Registration with full name",
                    value = json!({
                        "email": "kostiantyn.salnykov@gmail.com",
                        "phone": "+380671234567",
                        "password": "Fake123password!",
                        "first_name": "Kostiantyn",
                        "last_name": "Salnykov"
                    })
                )),
                ("2" = (
                    summary = "Registration without optional names",
                    value = json!({
                        "email": "kostiantyn.salnykov@gmail.com",
                        "password": "Fake123password!",
                    })
                )),
                ("3" = (
                    summary = "Registration by phone",
                    value = json!({
                        "phone": "+380978531216",
                        "password": "Fake123password!",
                    })
                ))
            )),
            (RegisterRequest = "application/msgpack")
        )
    ),
    responses(
        (
            status = 201,
            description = "User registered successfully",
            content(
                (crate::docs::schemas::AuthRegisterSuccessResponse = "application/json"),
                (crate::docs::schemas::AuthRegisterSuccessResponse = "application/msgpack")
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
            status = 409,
            description = "Conflict",
            content(
                (crate::docs::schemas::FailResponse = "application/json", example = json!({
                    "status": "fail",
                    "code": 409,
                    "message": "User with this email already exists."
                })),
                (crate::docs::schemas::FailResponse = "application/msgpack")
            )
        ),
        (
            status = 422,
            description = "Validation failed",
            content(
                (crate::docs::schemas::FailResponse = "application/json"),
                (crate::docs::schemas::FailResponse = "application/msgpack")
            ),
            examples(
                ("invalid_email" = (
                    summary = "Invalid email format",
                    value = json!({
                        "status": "fail",
                        "code": 422,
                        "message": "Invalid email format.",
                        "data": null
                    })
                )),
                ("weak_password" = (
                    summary = "Password too weak",
                    value = json!({
                        "status": "fail",
                        "code": 422,
                        "message": "Password must be at least 8 characters long.",
                        "data": null
                    })
                )),
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
fn auth_register_docs() {}
