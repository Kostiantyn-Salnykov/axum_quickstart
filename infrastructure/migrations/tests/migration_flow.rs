use app_config::Settings;
use migrations::Migrator;
use sea_orm_migration::MigratorTrait;
use sea_orm_migration::prelude::sea_orm::{
    ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, Statement,
};

#[tokio::test]
async fn migrations_can_move_forward_and_backward_one_step_at_a_time() {
    let settings = Settings::load().expect("failed to load settings");
    let db_name = format!("{}_migration_test", settings.postgres_db);

    recreate_database(&settings, &db_name)
        .await
        .expect("failed to recreate test database");

    let db = Database::connect(database_url_for(&settings, &db_name))
        .await
        .expect("failed to connect to test database");

    let total = Migrator::migrations().len();
    assert_eq!(pending_migrations_count(&db).await, total);

    for applied in 1..=total {
        Migrator::up(&db, Some(1))
            .await
            .expect("failed to apply next migration");

        assert_eq!(applied_migrations_count(&db).await, applied);
        assert_eq!(pending_migrations_count(&db).await, total - applied);
    }

    assert!(table_exists(&db, "users").await);
    assert!(table_exists(&db, "wishlists").await);
    assert!(table_exists(&db, "users_wishlists").await);

    for remaining in (0..total).rev() {
        Migrator::down(&db, Some(1))
            .await
            .expect("failed to roll back next migration");

        assert_eq!(applied_migrations_count(&db).await, remaining);
        assert_eq!(pending_migrations_count(&db).await, total - remaining);
    }

    assert!(!table_exists(&db, "users").await);
    assert!(!table_exists(&db, "wishlists").await);
    assert!(!table_exists(&db, "users_wishlists").await);

    drop(db);

    drop_database(&settings, &db_name)
        .await
        .expect("failed to clean up test database");
}

async fn recreate_database(settings: &Settings, db_name: &str) -> Result<(), DbErr> {
    drop_database(settings, db_name).await?;

    let admin = Database::connect(admin_database_url(settings)).await?;
    admin
        .execute_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!("CREATE DATABASE {}", quote_ident(db_name)),
        ))
        .await?;

    Ok(())
}

async fn drop_database(settings: &Settings, db_name: &str) -> Result<(), DbErr> {
    let admin = Database::connect(admin_database_url(settings)).await?;

    admin
        .execute_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}' AND pid <> pg_backend_pid()",
                quote_literal(db_name),
            ),
        ))
        .await?;

    admin
        .execute_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!("DROP DATABASE IF EXISTS {}", quote_ident(db_name)),
        ))
        .await?;

    Ok(())
}

async fn pending_migrations_count(db: &DatabaseConnection) -> usize {
    Migrator::get_pending_migrations(db)
        .await
        .expect("failed to fetch pending migrations")
        .len()
}

async fn applied_migrations_count(db: &DatabaseConnection) -> usize {
    if !table_exists(db, "migrations").await {
        return 0;
    }

    let row = db
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT COUNT(*)::bigint AS count FROM migrations".to_string(),
        ))
        .await
        .expect("failed to count applied migrations")
        .expect("count query returned no rows");

    let count: i64 = row.try_get("", "count").expect("count column is missing");
    count as usize
}

async fn table_exists(db: &DatabaseConnection, table_name: &str) -> bool {
    let row = db
        .query_one_raw(Statement::from_string(
            DatabaseBackend::Postgres,
            format!(
                "SELECT to_regclass('public.{}')::text AS name",
                quote_literal(table_name),
            ),
        ))
        .await
        .expect("failed to query table existence")
        .expect("existence query returned no rows");

    let name: Option<String> = row.try_get("", "name").expect("name column is missing");
    name.is_some()
}

fn admin_database_url(settings: &Settings) -> String {
    database_url_for(settings, "postgres")
}

fn database_url_for(settings: &Settings, db_name: &str) -> String {
    format!(
        "postgres://{username}:{password}@{host}:{port}/{db}",
        username = settings.postgres_user,
        password = settings.postgres_password,
        host = settings.postgres_host,
        port = settings.postgres_port,
        db = db_name,
    )
}

fn quote_ident(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn quote_literal(value: &str) -> String {
    value.replace('\'', "''")
}
