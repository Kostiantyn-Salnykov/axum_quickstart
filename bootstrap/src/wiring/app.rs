use std::sync::Arc;

use crate::wiring::auth::build_auth_services;
use crate::wiring::system::build_health_check_service;
use crate::wiring::users::{build_get_user_service, build_search_user_service};
use api_http::state::{AppState, SystemState, UsersState};
use app_config::Settings;
use application::auth::token_blacklist_port::TokenBlacklistPort;
use application::rate_limit::rate_limiter_port::RateLimiterPort;
use application::search::repository::SearchRepositoryPort;
use application::users::search::query::UserSearchField;
use application::users::search::result::UserSearchResult;
use infrastructure::adapters;
use infrastructure::adapters::redis::client::RedisClient;
use sea_orm::DatabaseConnection;

pub fn build_application_state(
    settings: &Settings,
    db: DatabaseConnection,
) -> Result<AppState, Box<dyn std::error::Error>> {
    let redis_client = RedisClient::new(&settings.redis_url())?;
    let blacklist: Arc<dyn TokenBlacklistPort> = Arc::new(
        adapters::redis::token_blacklist::RedisTokenBlacklistAdapter::new(redis_client.clone()),
    );
    let rate_limiter: Arc<dyn RateLimiterPort> = Arc::new(
        adapters::redis::rate_limiter::RedisRateLimiterAdapter::new(redis_client.clone()),
    );
    let token_manager = Arc::new(
        adapters::security::jwt_token_manager::JwtTokenManagerAdapter::new(
            settings.jwt_secret.as_bytes(),
            settings.access_token_ttl_minutes,
            settings.refresh_token_ttl_days,
            blacklist.clone(),
        ),
    );
    let health_provider = Arc::new(
        adapters::health::combined_health_check::CompositeHealthCheck::new(
            adapters::health::database_health_check::SeaOrmDatabaseHealthCheck::new(db.clone()),
            adapters::health::redis_health_check::RedisHealthCheck::new(redis_client),
        ),
    );
    let users_repo = Arc::new(
        adapters::persistence::seaorm_user_repository::SeaOrmUserRepositoryAdapter::new(db),
    );
    let password_hasher =
        Arc::new(adapters::security::argon_password_hasher::ArgonPasswordHasherAdapter);

    let health_check = build_health_check_service(health_provider);
    let auth = build_auth_services(
        users_repo.clone(),
        password_hasher,
        token_manager.clone(),
        blacklist,
    );
    let auth_users: Arc<dyn application::users::user_repository_port::UserRepositoryPort> =
        users_repo.clone();
    let search_users: Arc<dyn SearchRepositoryPort<UserSearchField, UserSearchResult>> =
        users_repo.clone();
    let get_user = build_get_user_service(auth_users);
    let search_user = build_search_user_service(search_users);

    Ok(AppState {
        rate_limiter,
        system: SystemState { health_check },
        auth,
        users: UsersState {
            get: get_user,
            search: search_user,
        },
    })
}
