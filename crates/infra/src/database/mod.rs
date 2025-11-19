use sea_orm::{ConnectOptions, Database};

pub mod checkout_repo;
pub mod product_repo;
pub mod user_repo;

/// Creates a database connection pool for the given database URL.
///
/// The function attempts to connect to the database identified by `url` and returns a
/// `sea_orm::DatabaseConnection` on success. This function will panic if the connection
/// attempt fails.
///
/// # Examples
///
/// ```no_run
/// # async {
/// let conn = create_connection_pool("sqlite::memory:").await;
/// // use `conn`...
/// # };
/// ```
pub async fn create_connection_pool(
    url: &str,
) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    let opt = ConnectOptions::new(url);
    Database::connect(opt).await
}
