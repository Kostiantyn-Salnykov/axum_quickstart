pub use sea_orm_migration::prelude::*;

mod m20260325_000001_create_users;
mod m20260412_000001_create_wishlists;
mod m20260412_000002_create_users_wishlists;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260325_000001_create_users::Migration),
            Box::new(m20260412_000001_create_wishlists::Migration),
            Box::new(m20260412_000002_create_users_wishlists::Migration),
        ]
    }

    fn migration_table_name() -> sea_orm::DynIden {
        "migrations".into_iden()
    }
}
