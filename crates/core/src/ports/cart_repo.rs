use async_trait::async_trait;
use entities::{
    cart::Model as CartModel, cart_item::Model as CartItemModel, product::Model as ProductModel,
};

use crate::error::Error;

#[async_trait]
pub trait CartRepository: Send + Sync {
    async fn create_new(&self, user_id: i64) -> Result<CartModel, Error>;

    async fn get_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<(CartModel, Vec<(CartItemModel, ProductModel)>)>, Error>;

    async fn update_item_quantity(
        &self,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<CartItemModel, Error>;

    async fn remove_cart_item(&self, user_id: i64, product_id: i64) -> Result<u64, Error>;
}
