use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .primary_key()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(User::Username).string_len(50).not_null())
                    .col(ColumnDef::new(User::Email).string_len(255).not_null())
                    .col(
                        ColumnDef::new(User::PasswordHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::FirstName).string_len(255).not_null())
                    .col(ColumnDef::new(User::LastName).string_len(255).not_null())
                    .col(ColumnDef::new(User::Address).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .big_integer()
                            .not_null()
                            .default(Expr::cust(
                                "CAST(EXTRACT(EPOCH FROM NOW()) * 1000 AS BIGINT)",
                            )),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    FirstName,
    LastName,
    Address,
    CreatedAt,
    UpdatedAt,
}
