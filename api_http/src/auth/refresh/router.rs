use crate::auth::refresh::handler::refresh;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/token/refresh/", post(refresh))
}
