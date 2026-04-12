use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wishlist::Table)
                    .if_not_exists()
                    .col(uuid(Wishlist::Id).not_null())
                    .col(string_len(Wishlist::Title, 128).not_null())
                    .col(string_len(Wishlist::Description, 256).not_null())
                    .col(integer(Wishlist::Priority).not_null())
                    .col(json_null(Wishlist::Settings))
                    .col(timestamp_with_time_zone(Wishlist::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Wishlist::UpdatedAt).not_null())
                    .primary_key(Index::create().name("pk_wishlists").col(Wishlist::Id))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wishlist::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Wishlist {
    #[sea_orm(iden = "wishlists")]
    Table,
    Id,
    Title,
    Description,
    Priority,
    Settings,
    CreatedAt,
    UpdatedAt,
}
