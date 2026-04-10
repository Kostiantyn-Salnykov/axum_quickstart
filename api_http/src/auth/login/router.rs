use crate::auth::login::handler::login;
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/auth/login/", post(login))
}
