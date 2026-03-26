use std::sync::Arc;

use infrastructure::{db::connection::connect_db, settings::Settings};
use service::services::health_check::HealthCheckService;
use service::services::register_user::RegisterUserService;
use service::use_cases::health_check::HealthCheckUseCase;
use service::use_cases::register_user::RegisterUserUseCase;
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
    let health_check: Arc<dyn HealthCheckUseCase> = Arc::new(HealthCheckService::new(provider));
    let register_user: Arc<dyn RegisterUserUseCase> =
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
