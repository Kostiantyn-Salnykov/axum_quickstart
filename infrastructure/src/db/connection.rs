use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

use crate::settings::Settings;

pub async fn connect_db(settings: &Settings) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(settings.database_url());

    options
        .max_connections(100)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5));

    Database::connect(options).await
}
