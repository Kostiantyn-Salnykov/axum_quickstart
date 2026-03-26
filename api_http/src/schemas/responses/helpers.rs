use crate::enums::JsendStatus;
use crate::schemas::responses::JsendResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

impl<T: Serialize> JsendResponse<T> {
    pub fn success(code: StatusCode, data: T, message: impl Into<String>) -> Self {
        Self {
            status: JsendStatus::Success,
            code: code.as_u16(),
            message: Some(message.into()),
            data: Some(data),
        }
    }

    pub fn fail(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: JsendStatus::Fail,
            code: code.as_u16(),
            message: Some(message.into()),
            data: None,
        }
    }

    pub fn error(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: JsendStatus::Error,
            code: code.as_u16(),
            message: Some(message.into()),
            data: None,
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
