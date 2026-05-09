mod wiring;

use app_config::Settings;
use infrastructure::adapters::persistence::seaorm_connection::connect_db;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use api_http::create_router;
use wiring::app::build_application_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    init_tracing(&settings.log_level)?;

    if settings.should_log_settings() {
        tracing::info!("{:#?}", settings);
    }
    let db = connect_db(&settings.database_url()).await?;
    let state = build_application_state(&settings, db)?;
    let app = create_router(state);

    tracing::info!(addr = %settings.server_addr(), "Starting HTTP server.");
    tracing::info!("Connecting to a database.");

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_new(log_level)?;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_target(true)
                .with_thread_ids(false)
                .compact(),
        )
        .try_init()?;

    Ok(())
}
