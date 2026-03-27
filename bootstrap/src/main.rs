use std::sync::Arc;

use application::ports::inbound::health_check::HealthCheck;
use application::ports::inbound::register_user::RegisterUser;
use application::services::health_check::HealthCheckService;
use application::services::register_user::RegisterUserService;
use infrastructure::{db::connection::connect_db, settings::Settings};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_http::{create_router, state::AppState};
use infrastructure::crypto::password::ArgonPasswordHasher;
use infrastructure::db::health_check::DbHealthCheckProvider;
use infrastructure::repositories::user::DbUserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
    println!("{:#?}", settings);
    let db = connect_db(&settings).await?;

    let provider = Arc::new(DbHealthCheckProvider::new(db.clone()));
    let users = Arc::new(DbUserRepository::new(db));
    let password_hasher = Arc::new(ArgonPasswordHasher);
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
