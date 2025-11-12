use std::net::SocketAddr;
use std::sync::Arc;

use app_core::service::user_service::UserService;
use infra::database::{create_connection_pool, user_repo::SeaOrmUserRepo};

use crate::config::Config;

mod config;
pub mod handlers;
mod router;
mod state;

/// Application entry point that bootstraps configuration, database connections, services, routing, and starts the HTTP server.
///
/// This function loads the configuration, creates the database connection pool, initializes structured logging,
/// constructs the repository and service layers, builds the application router with shared state, binds a TCP listener
/// on 0.0.0.0 at the configured port, and runs the Axum server until shutdown.
///
/// # Returns
///
/// `Ok(())` on clean shutdown; an error is returned if configuration loading, database pool creation, binding, or serving fails.
///
/// # Examples
///
/// ```no_run
/// // Start the server (run the compiled binary instead of executing in doc tests)
/// // $ cargo run --bin your_binary_name
/// ```
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new()?;
    let db_pool = create_connection_pool(&config.database.url).await;

    tracing_subscriber::fmt().json().init();

    let user_repo_adapter = Arc::new(SeaOrmUserRepo::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(user_repo_adapter));

    let app_state = state::AppState { user_service };
    let app = router::create_router(app_state);
    let addr = format!("0.0.0.0:{}", config.server.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}