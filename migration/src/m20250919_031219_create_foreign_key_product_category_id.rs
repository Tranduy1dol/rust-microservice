use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_product_category_id")
                    .from(Product::Table, Product::CategoryId)
                    .to(Category::Table, Category::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_product_category_id")
                    .table(Product::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Product {
    Table,
    CategoryId,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
}
