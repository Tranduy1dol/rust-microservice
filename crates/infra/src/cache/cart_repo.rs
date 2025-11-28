use app_core::dto::cart_dto::CartDto;
use app_core::error::Error;
use app_core::ports::cart_repo::CartRepository;
use async_trait::async_trait;
use redis::AsyncCommands;

use super::RedisPool;

pub struct RedisCartRepository {
    pool: RedisPool,
}

impl RedisCartRepository {
    /// Creates a new RedisCartRepository that uses the given RedisPool.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let pool = /* obtain a RedisPool */ unimplemented!();
    /// let repo = RedisCartRepository::new(pool);
    /// ```
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    /// Constructs the Redis key used to store a user's cart.
    ///
    /// Returns the Redis key string in the format `cart:{user_id}`, e.g. `cart:42`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let key = get_key(42);
    /// assert_eq!(key, "cart:42");
    /// ```
    fn get_key(user_id: i64) -> String {
        format!("cart:{}", user_id)
    }
}

#[async_trait]
impl CartRepository for RedisCartRepository {
    /// Retrieves the cached cart for the specified user from Redis.
    ///
    /// Attempts to fetch the JSON-encoded cart stored under the key `cart:{user_id}` and deserialize it into a `CartDto`.
    ///
    /// # Returns
    /// `Some(CartDto)` if a cart was found and successfully deserialized, `None` if no cart exists, or an `Error` for cache or deserialization failures.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example(repo: &RedisCartRepository) -> Result<(), Error> {
    /// let maybe_cart = repo.get_by_user_id(42).await?;
    /// if let Some(cart) = maybe_cart {
    ///     println!("Found cart for user {}", cart.user_id);
    /// }
    /// # Ok(()) }
    /// ```
    async fn get_by_user_id(&self, user_id: i64) -> Result<Option<CartDto>, Error> {
        let key = Self::get_key(user_id);
        let mut conn = self.pool.clone();

        let data: Option<String> = conn.get(key).await.map_err(|e| {
            tracing::error!("Redis error: {:?}", e);
            Error::internal("Cache error".to_string())
        })?;

        match data {
            Some(json) => serde_json::from_str(&json).map_err(|e| {
                tracing::error!("Deserialization error: {:?}", e);
                Error::internal("Data corruption".to_string())
            }),
            None => Ok(None),
        }
    }

    /// Saves the given user's cart in Redis under the key `cart:{user_id}` with a 24-hour TTL.
    ///
    /// Serializes `cart` to JSON and stores it in Redis; serialization or Redis errors are
    /// returned as `Error`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use crates::infra::cache::cart_repo::RedisCartRepository;
    /// # async fn example(pool: RedisPool, cart: CartDto) {
    /// let repo = RedisCartRepository::new(pool);
    /// repo.save(cart).await.unwrap();
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `Error` if serialization or the Redis operation fails.
    async fn save(&self, cart: CartDto) -> Result<(), Error> {
        let key = Self::get_key(cart.user_id);
        let mut conn = self.pool.clone();

        let json = serde_json::to_string(&cart).map_err(|e| {
            tracing::error!("Serialization error: {:?}", e);
            Error::internal("Serialization failed".to_string())
        })?;

        // Set with 24h expiration
        let _: () = conn.set_ex(key, json, 24 * 60 * 60).await.map_err(|e| {
            tracing::error!("Redis error: {:?}", e);
            Error::internal("Cache error".to_string())
        })?;

        Ok(())
    }

    /// Removes the cached cart for the specified user.
    ///
    /// Attempts to delete the Redis key associated with `user_id`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the key was deleted or did not exist, `Err(Error)` if the Redis operation failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example(repo: &crate::cache::RedisCartRepository) -> Result<(), crate::application::Error> {
    /// repo.delete_by_user_id(42).await?;
    /// # Ok(()) }
    /// ```
    async fn delete_by_user_id(&self, user_id: i64) -> Result<(), Error> {
        let key = Self::get_key(user_id);
        let mut conn = self.pool.clone();

        let _: () = conn.del(key).await.map_err(|e| {
            tracing::error!("Redis error: {:?}", e);
            Error::internal("Cache error".to_string())
        })?;

        Ok(())
    }
}
