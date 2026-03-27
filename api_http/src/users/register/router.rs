use crate::state::AppState;
use crate::users::register::handler::register_user;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/users/register/", post(register_user))
}
