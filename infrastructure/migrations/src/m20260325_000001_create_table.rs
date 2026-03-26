use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string_len(User::FirstName, 128).not_null())
                    .col(string_len(User::LastName, 128).not_null())
                    .col(string_len(User::Email, 256).not_null().unique_key())
                    .col(string_null(User::PasswordHash))
                    .col(string_len(User::Status, 32).not_null())
                    .col(string_len_null(User::Provider, 32))
                    .col(timestamp_with_time_zone(User::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(User::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // todo!();

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    PasswordHash,
    Status,
    Provider,
    CreatedAt,
    UpdatedAt,
}
