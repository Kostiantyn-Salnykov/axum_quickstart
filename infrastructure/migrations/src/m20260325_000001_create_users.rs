use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserStatus::Type)
                    .values([
                        UserStatus::Unconfirmed,
                        UserStatus::Confirmed,
                        UserStatus::ForceChangePassword,
                        UserStatus::WaitingForDeletion,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AuthProvider::Type)
                    .values([
                        AuthProvider::Google,
                        AuthProvider::Meta,
                        AuthProvider::Github,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(uuid(User::Id).not_null())
                    .col(string_len(User::FirstName, 128).not_null())
                    .col(string_len(User::LastName, 128).not_null())
                    .col(string_len(User::Email, 320).not_null())
                    .col(string_len_null(User::Phone, 20))
                    .col(string_null(User::PasswordHash))
                    .col(custom(User::Status, UserStatus::Type).not_null())
                    .col(custom_null(User::Provider, AuthProvider::Type))
                    .col(json_binary_null(User::Settings))
                    .col(timestamp_with_time_zone(User::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(User::UpdatedAt).not_null())
                    .primary_key(Index::create().name("pk_users").col(User::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uidx_users_email")
                    .table(User::Table)
                    .col(User::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("uidx_users_phone")
                    .table(User::Table)
                    .col(User::Phone)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("uidx_users_email").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("uidx_users_phone").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AuthProvider::Type).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserStatus::Type).to_owned())
            .await?;

        Ok(())
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
    Phone,
    PasswordHash,
    Status,
    Provider,
    Settings,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserStatus {
    #[sea_orm(iden = "users_status")]
    Type,
    #[sea_orm(iden = "unconfirmed")]
    Unconfirmed,
    #[sea_orm(iden = "confirmed")]
    Confirmed,
    #[sea_orm(iden = "force_change_password")]
    ForceChangePassword,
    #[sea_orm(iden = "waiting_for_deletion")]
    WaitingForDeletion,
}

#[derive(DeriveIden)]
enum AuthProvider {
    #[sea_orm(iden = "auth_provider")]
    Type,
    #[sea_orm(iden = "google")]
    Google,
    #[sea_orm(iden = "meta")]
    Meta,
    #[sea_orm(iden = "github")]
    Github,
}
