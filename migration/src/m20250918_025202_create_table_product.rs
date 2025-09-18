use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .big_integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Product::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Product::Description).string())
                    .col(
                        ColumnDef::new(Product::Price)
                            .decimal_len(10, 2)
                            .not_null()
                            .check(Expr::col(Product::Price).gte(0)),
                    )
                    .col(
                        ColumnDef::new(Product::StockQuantity)
                            .integer()
                            .not_null()
                            .check(Expr::col(Product::StockQuantity).gte(0)),
                    )
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Price,
    StockQuantity,
    CreatedAt,
    UpdatedAt,
}
