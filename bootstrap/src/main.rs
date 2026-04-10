use std::sync::Arc;

use application::auth::login::inbound::Login;
use application::auth::login::service::LoginService;
use application::auth::refresh::inbound::Refresh;
use application::auth::refresh::service::RefreshService;
use application::auth::register::inbound::Register;
use application::auth::register::service::RegisterService;
use application::system::health_check::inbound::HealthCheck;
use application::system::health_check::service::HealthCheckService;
use application::users::get::inbound::GetUser;
use application::users::get::service::GetUserService;
use infrastructure::{adapters, db::connection::connect_db, settings::Settings};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_http::{create_router, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("{:#?}", settings);
    let db = connect_db(&settings).await?;

    let token_manager = Arc::new(adapters::security::jwt_token_manager::JwtTokenManager::new(
        settings.jwt_secret.as_bytes(),
        settings.access_token_ttl_minutes,
        settings.refresh_token_ttl_days,
    ));
    let provider = Arc::new(
        adapters::health::database_health_check::SeaOrmDatabaseHealthCheck::new(db.clone()),
    );
    let users = Arc::new(adapters::persistence::user_repository::SeaOrmUserRepository::new(db));
    let password_hasher = Arc::new(adapters::security::password_hasher::ArgonPasswordHasher);
    let health_check: Arc<dyn HealthCheck> = Arc::new(HealthCheckService::new(provider));
    let auth_register: Arc<dyn Register> =
        Arc::new(RegisterService::new(users.clone(), password_hasher.clone()));
    let auth_login: Arc<dyn Login> = Arc::new(LoginService::new(
        users.clone(),
        password_hasher.clone(),
        token_manager.clone(),
    ));
    let auth_refresh: Arc<dyn Refresh> =
        Arc::new(RefreshService::new(users.clone(), token_manager.clone()));
    let get_user: Arc<dyn GetUser> = Arc::new(GetUserService::new(users, token_manager));

    let state = AppState {
        health_check,
        auth_register,
        auth_login,
        auth_refresh,
        get_user,
    };

    let app = create_router(state);

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
