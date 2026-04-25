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
                    .as_enum(UserWishlistRole::Type)
                    .values([
                        UserWishlistRole::Owner,
                        UserWishlistRole::Reader,
                        UserWishlistRole::Writer,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserWishlist::Table)
                    .if_not_exists()
                    .col(uuid(UserWishlist::UserId).not_null())
                    .col(uuid(UserWishlist::WishlistId).not_null())
                    .col(custom(UserWishlist::Role, UserWishlistRole::Type).not_null())
                    .col(timestamp_with_time_zone(UserWishlist::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(UserWishlist::UpdatedAt).not_null())
                    .primary_key(
                        Index::create()
                            .name("pk_users_wishlists")
                            .col(UserWishlist::UserId)
                            .col(UserWishlist::WishlistId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_wishlists_user_id")
                            .from(UserWishlist::Table, UserWishlist::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_wishlists_wishlist_id")
                            .from(UserWishlist::Table, UserWishlist::WishlistId)
                            .to(Wishlist::Table, Wishlist::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_wishlists_wishlist_id")
                    .table(UserWishlist::Table)
                    .col(UserWishlist::WishlistId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_users_wishlists_wishlist_id")
                    .table(UserWishlist::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserWishlist::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserWishlistRole::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserWishlist {
    #[sea_orm(iden = "users_wishlists")]
    Table,
    UserId,
    WishlistId,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Wishlist {
    #[sea_orm(iden = "wishlists")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserWishlistRole {
    #[sea_orm(iden = "users_wishlists_role")]
    Type,
    #[sea_orm(iden = "owner")]
    Owner,
    #[sea_orm(iden = "reader")]
    Reader,
    #[sea_orm(iden = "writer")]
    Writer,
}
