use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CartItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CartItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CartItem::CartId).big_integer().not_null())
                    .col(ColumnDef::new(CartItem::ProductId).big_integer().not_null())
                    .col(
                        ColumnDef::new(CartItem::Quantity)
                            .integer()
                            .not_null()
                            .check(Expr::col(CartItem::Quantity).gt(0)),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cart_item_cart_id_product_id")
                    .table(CartItem::Table)
                    .col(CartItem::CartId)
                    .col(CartItem::ProductId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_cart_item_cart_id")
                    .from(CartItem::Table, CartItem::CartId)
                    .to(Cart::Table, Cart::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_cart_item_product_id")
                    .from(CartItem::Table, CartItem::ProductId)
                    .to(Product::Table, Product::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CartItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CartItem {
    Table,
    Id,
    CartId,
    ProductId,
    Quantity,
}

#[derive(DeriveIden)]
enum Cart {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
