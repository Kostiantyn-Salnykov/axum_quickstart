mod wiring;

use app_config::Settings;
use infrastructure::adapters::persistence::seaorm_connection::connect_db;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_http::create_router;
use wiring::app::build_application_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
    if settings.should_log_settings() {
        tracing::info!("{:#?}", settings);
    }
    let db = connect_db(&settings.database_url()).await?;
    let state = build_application_state(&settings, db)?;
    let app = create_router(state);

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
