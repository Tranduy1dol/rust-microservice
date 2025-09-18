pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250918_024151_create_table_user::Migration),
            Box::new(m20250918_025202_create_table_product::Migration),
            Box::new(m20250918_025936_create_table_order::Migration),
            Box::new(m20250918_030902_create_table_order_item::Migration),
        ]
    }
}
mod m20250918_024151_create_table_user;
mod m20250918_025202_create_table_product;
mod m20250918_025936_create_table_order;
mod m20250918_030902_create_table_order_item;
