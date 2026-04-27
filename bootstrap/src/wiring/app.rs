use std::sync::Arc;

use crate::container::ApplicationContainer;
use crate::wiring::auth::build_auth_services;
use crate::wiring::system::build_health_check_service;
use crate::wiring::users::build_get_user_service;
use application::auth::token_blacklist_port::TokenBlacklistPort;
use infrastructure::adapters;
use infrastructure::adapters::cache::redis_client::RedisClient;
use infrastructure::settings::Settings;
use sea_orm::DatabaseConnection;

pub fn build_application_container(
    settings: &Settings,
    db: DatabaseConnection,
) -> ApplicationContainer {
    let redis_client =
        RedisClient::new(&settings.redis_url()).expect("Failed to create Redis client.");
    let blacklist: Arc<dyn TokenBlacklistPort> = Arc::new(
        adapters::cache::redis_token_blacklist::RedisTokenBlacklist::new(redis_client.clone()),
    );
    let token_manager = Arc::new(adapters::security::jwt_token_manager::JwtTokenManager::new(
        settings.jwt_secret.as_bytes(),
        settings.access_token_ttl_minutes,
        settings.refresh_token_ttl_days,
        blacklist.clone(),
    ));
    let health_provider = Arc::new(
        adapters::health::combined_health_check::CompositeHealthCheck::new(
            adapters::health::database_health_check::SeaOrmDatabaseHealthCheck::new(db.clone()),
            adapters::health::redis_health_check::RedisHealthCheck::new(redis_client),
        ),
    );
    let users =
        Arc::new(adapters::persistence::seaorm_user_repository::SeaOrmUserRepository::new(db));
    let password_hasher = Arc::new(adapters::security::argon_password_hasher::ArgonPasswordHasher);

    let health_check = build_health_check_service(health_provider);
    let auth = build_auth_services(
        users.clone(),
        password_hasher,
        token_manager.clone(),
        blacklist,
    );
    let get_user = build_get_user_service(users, token_manager.clone());

    ApplicationContainer::new(
        health_check,
        auth.register,
        auth.login,
        auth.logout,
        auth.refresh,
        token_manager.clone(),
        get_user,
    )
}
