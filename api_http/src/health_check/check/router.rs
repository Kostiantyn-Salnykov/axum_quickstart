use crate::health_check::check::handler::health_check;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/health_check/", get(health_check))
}
