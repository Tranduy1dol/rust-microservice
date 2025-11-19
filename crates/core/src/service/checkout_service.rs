use std::sync::Arc;

use entities::order;

use crate::error::Error;
use crate::ports::checkout_repo::CheckoutRepository;
use crate::service::cart_service::CartService;

pub struct CheckoutService {
    cart_service: Arc<CartService>,
    checkout_repo: Arc<dyn CheckoutRepository>,
}

impl CheckoutService {
    pub fn new(cart_service: Arc<CartService>, checkout_repo: Arc<dyn CheckoutRepository>) -> Self {
        Self {
            cart_service,
            checkout_repo,
        }
    }

    pub async fn checkout(&self, user_id: i64) -> Result<order::Model, Error> {
        let cart = self.cart_service.get_cart(user_id).await?;

        if cart.items.is_empty() {
            return Err(Error::bad_request("Cart is empty".to_string()));
        }

        let order = self.checkout_repo.create_order(user_id, cart.items).await?;

        // Clear cart after successful checkout
        self.cart_service.clear_cart(user_id).await?;

        Ok(order)
    }
}
