pub use sea_orm_migration::prelude::*;

mod m20260325_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260325_000001_create_table::Migration)]
    }

    fn migration_table_name() -> sea_orm::DynIden {
        "migrations".into_iden()
    }
}
