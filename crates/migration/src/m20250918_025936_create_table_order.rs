use sea_orm_migration::prelude::*;

use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(OrderStatusEnum::OrderStatus)
                    .values([
                        OrderStatusEnum::Placed,
                        OrderStatusEnum::Shipped,
                        OrderStatusEnum::Delivered,
                        OrderStatusEnum::Cancelled,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .big_integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Order::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .col(ColumnDef::new(Order::Status).enumeration(
                        OrderStatusEnum::OrderStatus,
                        [
                            OrderStatusEnum::Placed,
                            OrderStatusEnum::Shipped,
                            OrderStatusEnum::Delivered,
                            OrderStatusEnum::Cancelled,
                        ],
                    ))
                    .col(
                        ColumnDef::new(Order::TotalAmount)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_order_user_id")
                    .from(Order::Table, Order::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrderStatusEnum::OrderStatus).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    CreatedAt,
    Status,
    TotalAmount,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum OrderStatusEnum {
    #[iden = "order_status"]
    OrderStatus,
    #[iden = "placed"]
    Placed,
    #[iden = "shipped"]
    Shipped,
    #[iden = "delivered"]
    Delivered,
    #[iden = "cancelled"]
    Cancelled,
}
