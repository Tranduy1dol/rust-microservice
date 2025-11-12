use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Admin::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Admin::Name).string().not_null())
                    .col(ColumnDef::new(Admin::Email).string().not_null())
                    .col(ColumnDef::new(Admin::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(Admin::Level).integer().not_null().check(
                            Expr::col(Admin::Level)
                                .gte(1)
                                .and(Expr::col(Admin::Level).lte(3)),
                        ),
                    )
                    .col(
                        ColumnDef::new(Admin::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .col(
                        ColumnDef::new(Admin::IsActive)
                            .boolean()
                            .not_null()
                            .default("true"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Admin::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Admin {
    Table,
    Id,
    Name,
    Email,
    PasswordHash,
    Level,
    CreatedAt,
    IsActive,
}
