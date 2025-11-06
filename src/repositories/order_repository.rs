use sea_orm::DatabaseConnection;

pub struct OrderRepository {
    database: DatabaseConnection,
}

impl OrderRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}
