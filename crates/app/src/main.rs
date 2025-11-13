use std::net::SocketAddr;
use std::sync::Arc;

use app_core::events::AppEvent;
use app_core::service::{product_service::ProductService, user_service::UserService};
use app_core::tracing::init_standard_tracing;
use infra::database::{
    create_connection_pool, product_repo::SeaOrmProductRepo, user_repo::SeaOrmUserRepo,
};
use tokio::sync::mpsc;

use crate::config::Config;

mod config;
pub mod handlers;
mod router;
mod state;
mod worker;

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

    init_standard_tracing(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_NAME"),
        &config.log.level,
    );

    let db_pool = create_connection_pool(&config.database.url).await?;
    tracing::info!("Database connected successfully");

    let (event_sender, event_receiver) = mpsc::channel::<AppEvent>(100);
    tokio::spawn(worker::event_worker(event_receiver));

    let user_repo_adapter = Arc::new(SeaOrmUserRepo::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(
        user_repo_adapter,
        config.jwt.secret,
        event_sender,
    ));

    let product_repo_adapter = Arc::new(SeaOrmProductRepo::new(db_pool.clone()));
    let product_service = Arc::new(ProductService::new(product_repo_adapter));

    let app_state = state::AppState {
        user_service,
        product_service,
    };
    let app = router::create_router(app_state);
    let addr = format!("0.0.0.0:{}", config.server.port);

    tracing::info!("Server listening on addr: {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
