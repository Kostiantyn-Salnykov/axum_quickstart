use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JsendStatus {
    Success,
    Fail,
    Error,
}
