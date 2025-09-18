use std::sync::LazyLock;

use sea_orm::DatabaseConnection;

use crate::database::get_database;

pub struct OrderRepository {
    database: DatabaseConnection,
}

impl OrderRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }
}

pub static ORDER_REPOSITORY: LazyLock<OrderRepository> = LazyLock::new(OrderRepository::new);
