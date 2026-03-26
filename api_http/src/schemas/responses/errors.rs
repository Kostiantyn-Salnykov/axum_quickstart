use crate::enums::JsendStatus;
use crate::errors::AppError;
use crate::schemas::responses::JsendResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (http_status, jsend_status, message) = match &self {
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
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                JsendStatus::Error,
                e.to_string(),
            ),
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
