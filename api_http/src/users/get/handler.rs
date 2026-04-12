use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::get::response::UserResponse;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.users.get.get_by_id(id).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        UserResponse::from(result),
        "User fetched successfully.",
    ))
}

pub async fn get_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = bearer_token(&headers)?;
    let result = state.users.get.get_me(token.to_string()).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        UserResponse::from(result),
        "Current user fetched successfully.",
    ))
}

fn bearer_token(headers: &HeaderMap) -> Result<&str, AppError> {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    header
        .strip_prefix("Bearer ")
        .or_else(|| header.strip_prefix("bearer "))
        .ok_or(AppError::Unauthorized)
}
