use std::sync::Arc;

use crate::dto::cart_dto::{CartDto, CartItemDto};
use crate::error::Error;
use crate::ports::cart_repo::CartRepository;

pub struct CartService {
    cart_repo: Arc<dyn CartRepository>,
}

impl CartService {
    pub fn new(cart_repo: Arc<dyn CartRepository>) -> Self {
        Self { cart_repo }
    }

    pub async fn get_cart(&self, user_id: i64) -> Result<CartDto, Error> {
        let cart = self.cart_repo.get_by_user_id(user_id).await?;
        Ok(cart.unwrap_or_else(|| CartDto::new(user_id)))
    }

    pub async fn add_item(
        &self,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<CartDto, Error> {
        if quantity <= 0 {
            return Err(Error::bad_request("Quantity must be positive".to_string()));
        }

        let mut cart = self.get_cart(user_id).await?;

        if let Some(item) = cart.items.iter_mut().find(|i| i.product_id == product_id) {
            item.quantity += quantity;
        } else {
            cart.items.push(CartItemDto {
                product_id,
                quantity,
            });
        }

        self.cart_repo.save(cart.clone()).await?;
        Ok(cart)
    }

    pub async fn remove_item(&self, user_id: i64, product_id: i64) -> Result<CartDto, Error> {
        let mut cart = self.get_cart(user_id).await?;

        cart.items.retain(|i| i.product_id != product_id);

        self.cart_repo.save(cart.clone()).await?;
        Ok(cart)
    }

    pub async fn clear_cart(&self, user_id: i64) -> Result<(), Error> {
        self.cart_repo.delete_by_user_id(user_id).await
    }
}
