use std::net::SocketAddr;
use std::sync::Arc;

use app_core::service::user_service::UserService;
use infra::database::{create_connection_pool, user_repo::SeaOrmUserRepo};

use crate::config::Config;

mod config;
pub mod handlers;
mod router;
mod state;

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
