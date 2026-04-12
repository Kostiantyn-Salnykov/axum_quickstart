use std::sync::Arc;

use crate::container::ApplicationContainer;
use crate::wiring::auth::build_auth_services;
use crate::wiring::system::build_health_check_service;
use crate::wiring::users::build_get_user_service;
use infrastructure::adapters;
use infrastructure::settings::Settings;
use sea_orm::DatabaseConnection;

pub fn build_application_container(
    settings: &Settings,
    db: DatabaseConnection,
) -> ApplicationContainer {
    let token_manager = Arc::new(adapters::security::jwt_token_manager::JwtTokenManager::new(
        settings.jwt_secret.as_bytes(),
        settings.access_token_ttl_minutes,
        settings.refresh_token_ttl_days,
    ));
    let health_provider = Arc::new(
        adapters::health::database_health_check::SeaOrmDatabaseHealthCheck::new(db.clone()),
    );
    let users = Arc::new(adapters::persistence::user_repository::SeaOrmUserRepository::new(db));
    let password_hasher = Arc::new(adapters::security::password_hasher::ArgonPasswordHasher);

    let health_check = build_health_check_service(health_provider);
    let auth = build_auth_services(users.clone(), password_hasher, token_manager.clone());
    let get_user = build_get_user_service(users, token_manager);

    ApplicationContainer::new(
        health_check,
        auth.register,
        auth.login,
        auth.refresh,
        get_user,
    )
}
