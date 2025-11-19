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
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    fn get_key(user_id: i64) -> String {
        format!("cart:{}", user_id)
    }
}

#[async_trait]
impl CartRepository for RedisCartRepository {
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
