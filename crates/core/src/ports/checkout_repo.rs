use async_trait::async_trait;
use entities::order::Model as OrderModel;

use crate::dto::cart_dto::CartItemDto;
use crate::error::Error;

#[async_trait]
pub trait CheckoutRepository: Send + Sync {
    async fn create_order(
        &self,
        user_id: i64,
        items: Vec<CartItemDto>,
    ) -> Result<OrderModel, Error>;
}
