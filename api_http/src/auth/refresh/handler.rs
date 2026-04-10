use crate::auth::refresh::request::RefreshRequest;
use crate::auth::refresh::response::RefreshResponse;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn refresh(
    State(state): State<AppState>,
    payload: Result<Json<RefreshRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(AppError::from_json_rejection)?;
    let result = state.auth_refresh.refresh(payload.refresh_token).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        RefreshResponse::from(result),
        "Tokens refreshed successfully.",
    ))
}
