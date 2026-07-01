use crate::auth::refresh::request::RefreshRequest;
use crate::auth::refresh::response::RefreshResponse;
use crate::content::ContentBody;
use crate::errors::AppError;
use crate::responses::JsendResponse;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn refresh(
    State(state): State<AppState>,
    payload: Result<ContentBody<RefreshRequest>, crate::content::ContentBodyRejection>,
) -> Result<impl IntoResponse, AppError> {
    let ContentBody(payload) = payload.map_err(AppError::from_content_body_rejection)?;
    let result = state.auth.refresh.refresh(payload.refresh_token).await?;

    Ok(JsendResponse::success(
        StatusCode::OK,
        RefreshResponse::from(result),
        "Tokens refreshed successfully.",
    ))
}
