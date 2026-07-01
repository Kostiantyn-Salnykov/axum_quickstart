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
                    .col(string_len(Wishlist::Title, 127).not_null())
                    .col(string_len(Wishlist::Description, 256).not_null())
                    .col(small_integer(Wishlist::Priority).not_null())
                    .col(json_binary_null(Wishlist::Settings))
                    .col(timestamp_with_time_zone(Wishlist::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Wishlist::UpdatedAt).not_null())
                    .col(uuid(Wishlist::CreatedBy).not_null())
                    .col(uuid(Wishlist::UpdatedBy).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wishlists_created_by")
                            .from(Wishlist::Table, Wishlist::CreatedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wishlists_updated_by")
                            .from(Wishlist::Table, Wishlist::UpdatedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(Index::create().name("pk_wishlists").col(Wishlist::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wishlists_created_by")
                    .table(Wishlist::Table)
                    .col(Wishlist::CreatedBy)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_wishlists_updated_by")
                    .table(Wishlist::Table)
                    .col(Wishlist::UpdatedBy)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_wishlists_updated_by")
                    .table(Wishlist::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_wishlists_created_by")
                    .table(Wishlist::Table)
                    .to_owned(),
            )
            .await?;

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
    CreatedBy,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}
