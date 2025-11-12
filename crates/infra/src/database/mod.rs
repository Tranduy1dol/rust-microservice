use sea_orm::{ConnectOptions, Database};

pub mod user_repo;

pub async fn create_connection_pool(url: &str) -> sea_orm::DatabaseConnection {
    let opt = ConnectOptions::new(url);
    Database::connect(opt).await.unwrap()
}
