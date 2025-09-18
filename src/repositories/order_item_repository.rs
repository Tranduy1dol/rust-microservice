use std::sync::LazyLock;

use sea_orm::DatabaseConnection;

use crate::database::get_database;

pub struct OrderItemRepository {
    database: DatabaseConnection,
}

impl OrderItemRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }
}

pub static ORDER_ITEM_REPOSITORY: LazyLock<OrderItemRepository> =
    LazyLock::new(OrderItemRepository::new);
