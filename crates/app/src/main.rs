use std::net::SocketAddr;
use std::sync::Arc;

use app_core::{
    events::AppEvent,
    service::{
        auth_service::AuthService, cart_service::CartService, checkout_service::CheckoutService,
        product_service::ProductService, user_service::UserService,
    },
    tracing::init_standard_tracing,
};
use infra::{
    cache::RedisCartRepository,
    database::{
        checkout_repo::SeaOrmCheckoutRepo, create_connection_pool, product_repo::SeaOrmProductRepo,
        user_repo::SeaOrmUserRepo,
    },
};
use tokio::sync::mpsc;

use app_lib::{config::Config, router, state, worker};

/// Bootstraps the application and runs the HTTP server.
///
/// Loads configuration, initializes observability, creates database and Redis pools,
/// constructs repositories and services, assembles shared application state, builds the HTTP router,
/// binds a TCP listener on `0.0.0.0:<port>` from the configuration, and runs the Axum server until shutdown.
///
/// # Returns
///
/// `Ok(())` on clean shutdown; an error if configuration loading, pool creation, binding, or serving fails.
///
/// # Examples
///
/// ```no_run
/// // Run the compiled binary to start the server:
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

    let redis_pool = infra::cache::create_connection_pool(&config.redis.url).await?;
    tracing::info!("Redis connected successfully");

    let (event_sender, event_receiver) = mpsc::channel::<AppEvent>(100);
    tokio::spawn(worker::event_worker(event_receiver));

    let user_repo_adapter = Arc::new(SeaOrmUserRepo::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(
        user_repo_adapter,
        config.jwt.secret.clone(),
        event_sender,
    ));

    let auth_service = Arc::new(AuthService::new(config.jwt.secret));

    let product_repo_adapter = Arc::new(SeaOrmProductRepo::new(db_pool.clone()));
    let product_service = Arc::new(ProductService::new(product_repo_adapter));

    let cart_repo_adapter = Arc::new(RedisCartRepository::new(redis_pool));
    let cart_service = Arc::new(CartService::new(cart_repo_adapter));

    let checkout_repo_adapter = Arc::new(SeaOrmCheckoutRepo::new(db_pool.clone()));
    let checkout_service = Arc::new(CheckoutService::new(
        cart_service.clone(),
        checkout_repo_adapter,
    ));

    let app_state = state::AppState {
        auth_service,
        user_service,
        product_service,
        cart_service,
        checkout_service,
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
