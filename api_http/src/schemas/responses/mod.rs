use crate::enums::JsendStatus;
use serde::Serialize;

mod errors;
pub(crate) mod health_check;
pub(crate) mod helpers;
pub(crate) mod users;

#[derive(Serialize)]
pub struct JsendResponse<T: Serialize> {
    pub status: JsendStatus,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
