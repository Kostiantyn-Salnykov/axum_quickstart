mod container;
mod wiring;

use api_http::ApiServices;
use infrastructure::{adapters::persistence::seaorm_connection::connect_db, settings::Settings};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api_http::create_router;
use wiring::app::build_application_container;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&settings.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("{:#?}", settings);
    let db = connect_db(&settings).await?;
    let container = build_application_container(&settings, db);
    let app = create_router(ApiServices {
        system_health_check: container.system.health_check,
        auth_register: container.auth.register,
        auth_login: container.auth.login,
        auth_logout: container.auth.logout,
        auth_refresh: container.auth.refresh,
        auth_token_manager: container.auth.token_manager,
        users_get: container.users.get,
    });

    let listener = TcpListener::bind(settings.server_addr()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
