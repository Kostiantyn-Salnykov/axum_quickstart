#![allow(dead_code)]
use crate::users::search::request::UserSearchRequest;

#[utoipa::path(
    post,
    path = "/v1/users/search/",
    tag = "users",
    request_body(
        content = UserSearchRequest,
        description = "Search users with full-text search, filters, projection, sorting and pagination.",
        content_type = "application/json",
        examples(
            ("1" = (
                summary = "Search by name and email",
                value = json!({
                    "searching": {
                        "value": "kostiantyn",
                        "fields": ["first_name", "last_name", "email"]
                    },
                    "projection": {
                        "mode": "show",
                        "fields": ["id", "email", "first_name", "last_name", "status"]
                    },
                    "filtration": {
                        "kind": "group",
                        "combinator": "and",
                        "items": [
                            {
                                "kind": "condition",
                                "field": "status",
                                "operator": "eq",
                                "values": ["confirmed"]
                            }
                        ]
                    },
                    "sorting": [
                        { "field": "created_at", "direction": "desc" },
                        { "field": "id", "direction": "desc" }
                    ],
                    "pagination": {
                        "kind": "page_size",
                        "page": 1,
                        "size": 25
                    }
                })
            ))
        )
    ),
    responses(
        (
            status = 200,
            description = "Users fetched successfully.",
            body = crate::docs::schemas::UsersSearchSuccessResponse
        ),
        (
            status = 400,
            description = "Bad request",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 422,
            description = "Validation failed",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 500,
            description = "Internal server error",
            body = crate::docs::schemas::ErrorResponse
        )
    )
)]
fn users_search_docs() {}

#[utoipa::path(
    post,
    path = "/v1/users/search/stream",
    tag = "users",
    request_body(
        content = UserSearchRequest,
        description = "Stream matching users as NDJSON.",
        content_type = "application/json"
    ),
    responses(
        (
            status = 200,
            description = "NDJSON stream of matching users.",
            body = String,
            content_type = "application/x-ndjson"
        ),
        (
            status = 400,
            description = "Bad request",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 422,
            description = "Validation failed",
            body = crate::docs::schemas::FailResponse
        ),
        (
            status = 500,
            description = "Internal server error",
            body = crate::docs::schemas::ErrorResponse
        )
    )
)]
fn users_search_stream_docs() {}
