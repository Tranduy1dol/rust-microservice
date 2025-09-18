use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItem::Id)
                            .not_null()
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItem::OrderId).big_integer().not_null())
                    .col(
                        ColumnDef::new(OrderItem::ProductId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItem::Quantity)
                            .integer()
                            .not_null()
                            .check(Expr::col(OrderItem::Quantity).gt(0)),
                    )
                    .col(
                        ColumnDef::new(OrderItem::PricePerUnit)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_order_item_order_id_product_id")
                    .table(OrderItem::Table)
                    .col(OrderItem::OrderId)
                    .col(OrderItem::ProductId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderItem {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity,
    PricePerUnit,
}
