use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cart::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Cart::UserId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .col(
                        ColumnDef::new(Cart::UpdatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_cart_user_id")
                    .from(Cart::Table, Cart::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cart::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Cart {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
