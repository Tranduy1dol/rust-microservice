use sea_orm::DatabaseConnection;

pub struct OrderItemRepository {
    database: DatabaseConnection,
}

impl OrderItemRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}
