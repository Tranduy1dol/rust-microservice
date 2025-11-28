pub mod cart_repo;

pub use cart_repo::RedisCartRepository;

type RedisPool = redis::aio::ConnectionManager;

/// Create a Redis connection manager from the given Redis URL.
///
/// Returns `Ok(RedisPool)` with a connection manager on success, `Err(redis::RedisError)` on failure.
///
/// # Examples
///
/// ```ignore
/// // Example uses a synchronous runtime to call the async function.
/// let pool = tokio::runtime::Runtime::new()
///     .unwrap()
///     .block_on(async { crate::cache::create_connection_pool("redis://127.0.0.1/").await });
///
/// match pool {
///     Ok(_mgr) => { /* connection manager obtained */ }
///     Err(e) => panic!("failed to create redis pool: {}", e),
/// }
/// ```
pub async fn create_connection_pool(url: &str) -> Result<RedisPool, redis::RedisError> {
    let redis_client = redis::Client::open(url)?;
    redis_client.get_connection_manager().await
}