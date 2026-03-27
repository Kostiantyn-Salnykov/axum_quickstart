use crate::enums::JsendStatus;
use crate::errors::AppError;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsendResponse<T: Serialize> {
    pub status: JsendStatus,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> JsendResponse<T> {
    pub fn success(code: StatusCode, data: T, message: impl Into<String>) -> Self {
        Self {
            status: JsendStatus::Success,
            code: code.as_u16(),
            message: Some(message.into()),
            data: Some(data),
        }
    }
}

impl<T: Serialize> IntoResponse for JsendResponse<T> {
    fn into_response(self) -> Response {
        let http_status =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (http_status, Json(self)).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (http_status, jsend_status, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, JsendStatus::Fail, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, JsendStatus::Fail, msg.clone()),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                JsendStatus::Fail,
                msg.clone(),
            ),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, JsendStatus::Fail, msg.clone()),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                JsendStatus::Error,
                "Unauthorized".to_string(),
            ),
            AppError::Internal(error) => {
                tracing::error!(error = ?error, "Unhandled internal application error.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsendStatus::Error,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = JsendResponse::<()> {
            status: jsend_status,
            code: http_status.as_u16(),
            message: Some(message),
            data: None,
        };

        (http_status, Json(body)).into_response()
    }
}
