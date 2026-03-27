use std::sync::Arc;

use application::system::health_check::inbound::HealthCheck;
use application::system::health_check::service::HealthCheckService;
use application::users::register::inbound::RegisterUser;
use application::users::register::service::RegisterUserService;
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

    let provider = Arc::new(
        adapters::health::database_health_check::SeaOrmDatabaseHealthCheck::new(db.clone()),
    );
    let users = Arc::new(adapters::persistence::user_repository::SeaOrmUserRepository::new(db));
    let password_hasher = Arc::new(adapters::security::password_hasher::ArgonPasswordHasher);
    let health_check: Arc<dyn HealthCheck> = Arc::new(HealthCheckService::new(provider));
    let register_user: Arc<dyn RegisterUser> =
        Arc::new(RegisterUserService::new(users, password_hasher));

    let state = AppState {
        health_check,
        register_user,
    };

    let app = create_router(state);

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
