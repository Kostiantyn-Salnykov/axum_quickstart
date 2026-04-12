use std::sync::Arc;

use api_http::state::{AppState, AuthState, SystemState, UsersState};
use application::auth::login::inbound::Login;
use application::auth::refresh::inbound::Refresh;
use application::auth::register::inbound::Register;
use application::system::health_check::inbound::HealthCheck;
use application::users::get::inbound::GetUser;

pub struct ApplicationContainer {
    pub state: AppState,
}

impl ApplicationContainer {
    pub fn new(
        health_check: Arc<dyn HealthCheck>,
        auth_register: Arc<dyn Register>,
        auth_login: Arc<dyn Login>,
        auth_refresh: Arc<dyn Refresh>,
        get_user: Arc<dyn GetUser>,
    ) -> Self {
        Self {
            state: AppState {
                system: SystemState { health_check },
                auth: AuthState {
                    register: auth_register,
                    login: auth_login,
                    refresh: auth_refresh,
                },
                users: UsersState { get: get_user },
            },
        }
    }
}
