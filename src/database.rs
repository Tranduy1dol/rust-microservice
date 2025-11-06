use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use crate::config::DbConfig;

pub static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn setup_database_connection(
    database_url: &str,
    db_config: &Option<DbConfig>,
) -> Result<DatabaseConnection, DbErr> {
    connect_to_database(database_url, db_config.clone()).await
}

async fn connect_to_database(
    database_url: &str,
    db_config: Option<DbConfig>,
) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    let db_config = db_config.unwrap_or_default();
    opt.max_connections(db_config.max_connections.max(1))
        .min_connections(db_config.min_connections)
        .connect_timeout(Duration::from_secs(db_config.connect_timeout.max(1)))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout.max(1)))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout.max(60)))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime.max(300)))
        .sqlx_logging(db_config.enable_query_log);

    Database::connect(opt).await
}
