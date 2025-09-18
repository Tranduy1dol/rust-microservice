use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use crate::config::DbConfig;

pub static DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn setup_database_connection(
    database_url: &str,
    db_config: &Option<DbConfig>,
) -> anyhow::Result<()> {
    match connect_to_database(database_url, db_config.clone()).await {
        Ok(db) => {
            DATABASE.set(db).expect("Database already set");
            Ok(())
        }
        Err(err) => {
            println!("Database connection error: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn get_database() -> &'static DatabaseConnection {
    DATABASE.get().expect("Database not set")
}

async fn connect_to_database(
    database_url: &str,
    db_config: Option<DbConfig>,
) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    let db_config = db_config.unwrap_or_default();
    opt.max_connections(db_config.max_connections.max(1))
        .min_connections(db_config.min_connections.max(0))
        .connect_timeout(Duration::from_secs(db_config.connect_timeout.max(1)))
        .acquire_timeout(Duration::from_secs(db_config.acquire_timeout.max(1)))
        .idle_timeout(Duration::from_secs(db_config.idle_timeout.max(60)))
        .max_lifetime(Duration::from_secs(db_config.max_lifetime.max(300)))
        .sqlx_logging(db_config.enable_query_log);

    Database::connect(opt).await
}
