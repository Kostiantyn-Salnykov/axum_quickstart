use crate::errors::AppError;
use crate::middlewares::VerifiedToken;
use crate::responses::JsendResponse;
use crate::state::AppState;
use crate::users::get::response::UserResponse;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
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
    Extension(verified): Extension<VerifiedToken>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.users.get.get_me(verified.0.user_id).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        UserResponse::from(result),
        "Current user fetched successfully.",
    ))
}
