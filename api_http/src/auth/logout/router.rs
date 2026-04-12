use crate::auth::logout::handler::logout;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/logout/", post(logout))
}
