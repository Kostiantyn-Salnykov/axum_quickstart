use std::sync::Arc;

use infrastructure::{db::connection::connect_db, settings::Settings};
use service::services::health_check::HealthCheckService;
use service::use_cases::health_check::HealthCheckUseCase;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_http::{create_router, state::AppState};
use infrastructure::db::health_check::DbHealthCheckProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
    println!("{:#?}", settings);
    let db = connect_db(&settings).await?;

    let provider = Arc::new(DbHealthCheckProvider::new(db));
    let health_check: Arc<dyn HealthCheckUseCase> = Arc::new(HealthCheckService::new(provider));

    let state = AppState { health_check };

    let app = create_router(state);

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
