use crate::users::register::request::RegisterUserRequest;

#[utoipa::path(
    post,
    path = "/v1/users/register/",
    tag = "users",
    request_body(
        content = RegisterUserRequest,
        description = "User registration payload",
        content_type = "application/json",
        examples(
            ("1" = (
                summary = "Registration with full name",
                value = json!({
                    "email": "kostiantyn.salnykov@gmail.com",
                    "password": "fake123password!",
                    "first_name": "Kostiantyn",
                    "last_name": "Salnykov"
                })
            )),
            ("2" = (
                summary = "Registration without optional names",
                value = json!({
                    "email": "kostiantyn.salnykov@gmail.com",
                    "password": "fake123password!",
                    "first_name": null,
                    "last_name": null
                })
            ))
        )
    ),
    responses(
        (
            status = 201,
            description = "User registered successfully",
            body = crate::docs::schemas::RegisterUserSuccessResponse
        ),
        (
            status = 400,
            description = "Bad request",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 409,
            description = "Conflict",
            body = crate::docs::schemas::FailResponse,
            example = json!({
                "status": "fail",
                "code": 409,
                "message": "User with this email already exists."
            })
        ),
        (
            status = 422,
            description = "Validation failed",
            body = crate::docs::schemas::FailResponse,
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
            body = crate::docs::schemas::ErrorResponse
        )
    )
)]
fn register_user_docs() {}
