use axum::http;
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

#[derive(Clone)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        let id = Uuid::now_v7().to_string();
        let header_value = id.parse().ok()?;
        Some(RequestId::new(header_value))
    }
}
