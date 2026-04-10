use crate::auth::register::handler::register;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/register/", post(register))
}
