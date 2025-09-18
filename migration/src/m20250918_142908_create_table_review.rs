use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::ProductId).big_integer().not_null())
                    .col(ColumnDef::new(Review::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Review::Rating).integer().not_null().check(
                            Expr::col(Review::Rating)
                                .gte(1)
                                .and(Expr::col(Review::Rating).lte(5)),
                        ),
                    )
                    .col(ColumnDef::new(Review::Comment).string())
                    .col(
                        ColumnDef::new(Review::CreatedAt)
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
                    .name("fk_review_user_id")
                    .from(Review::Table, Review::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_review_product_id")
                    .from(Review::Table, Review::ProductId)
                    .to(Product::Table, Product::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_review_product_id_user_id")
                    .table(Review::Table)
                    .col(Review::UserId)
                    .col(Review::ProductId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Review {
    Table,
    Id,
    ProductId,
    UserId,
    Rating,
    Comment,
    CreatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}
