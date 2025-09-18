use std::sync::LazyLock;

use sea_orm::DatabaseConnection;

use crate::database::get_database;

pub struct ProductRepository {
    database: DatabaseConnection,
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }
}

pub static PRODUCT_REPOSITORY: LazyLock<ProductRepository> = LazyLock::new(ProductRepository::new);
