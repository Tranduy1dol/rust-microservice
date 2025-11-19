use async_trait::async_trait;

use crate::dto::cart_dto::CartDto;
use crate::error::Error;

#[async_trait]
pub trait CartRepository: Send + Sync {
    async fn get_by_user_id(&self, user_id: i64) -> Result<Option<CartDto>, Error>;
    async fn save(&self, cart: CartDto) -> Result<(), Error>;
    async fn delete_by_user_id(&self, user_id: i64) -> Result<(), Error>;
}
