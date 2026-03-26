use infrastructure::settings::Settings;
use migrations::Migrator;
use migrations::sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() {
    let settings = Settings::load().expect("Failed to load settings");
    let db = Database::connect(settings.database_url())
        .await
        .expect("Failed to connect to a database.");

    let command = std::env::args().nth(1).unwrap_or("up".to_string());
    let steps = std::env::args().nth(2).and_then(|s| s.parse::<u32>().ok());

    match command.as_str() {
        "up" => Migrator::up(&db, steps).await.expect("Migration failed!"),
        "down" => Migrator::down(&db, steps).await.expect("Rollback failed!"),
        "fresh" => Migrator::fresh(&db).await.expect("Fresh failed!"),
        "reset" => Migrator::reset(&db).await.expect("Reset failed!"),
        "status" => {
            let statuses = Migrator::get_pending_migrations(&db)
                .await
                .expect("Failed to get status");
            for m in statuses {
                println!("Pending: {}", m.name());
            }
        }
        _ => eprintln!(
            "Unknown command: {}. Use: up | down | fresh | reset | status!",
            command
        ),
    }
}
