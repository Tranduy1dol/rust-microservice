pub mod cart_repo;

pub use cart_repo::RedisCartRepository;

type RedisPool = redis::aio::ConnectionManager;

pub async fn create_connection_pool(url: &str) -> Result<RedisPool, redis::RedisError> {
    let redis_client = redis::Client::open(url)?;
    redis_client.get_connection_manager().await
}
