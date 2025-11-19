use app_core::dto::cart_dto::CartItemDto;
use app_core::error::Error;
use app_core::ports::checkout_repo::CheckoutRepository;
use async_trait::async_trait;
use entities::{order, order_item, product};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};

pub struct SeaOrmCheckoutRepo {
    db: DatabaseConnection,
}

impl SeaOrmCheckoutRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CheckoutRepository for SeaOrmCheckoutRepo {
    async fn create_order(
        &self,
        user_id: i64,
        items: Vec<CartItemDto>,
    ) -> Result<order::Model, Error> {
        let txn = self.db.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {:?}", e);
            Error::internal("Transaction error".to_string())
        })?;

        // 1. Create Order with 0 total first (or calculate first)
        // To calculate first, we need to iterate items. But we also need to lock them.
        // So we can create order with 0, then update it after processing items.

        use entities::sea_orm_active_enums::OrderStatus;

        // ... imports ...

        let order = order::ActiveModel {
            user_id: Set(user_id),
            total_amount: Set(Decimal::ZERO),
            order_status: Set(Some(OrderStatus::Placed)),
            order_date: Set(chrono::Utc::now().timestamp()),
            ..Default::default()
        };

        let order_model = order.insert(&txn).await.map_err(|e| {
            tracing::error!("Failed to create order: {:?}", e);
            Error::internal("Order creation failed".to_string())
        })?;

        let mut total_amount = Decimal::ZERO;

        // 2. Process Items
        for item in items {
            let product = product::Entity::find_by_id(item.product_id)
                .one(&txn)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to find product: {:?}", e);
                    Error::internal("Database error".to_string())
                })?
                .ok_or_else(|| Error::not_found("Product not found".to_string()))?;

            if product.stock_quantity < item.quantity {
                return Err(Error::bad_request(format!(
                    "Insufficient stock for product {}",
                    product.name
                )));
            }

            // Update stock
            let mut product_active: product::ActiveModel = product.clone().into();
            product_active.stock_quantity = Set(product.stock_quantity - item.quantity);
            product_active.update(&txn).await.map_err(|e| {
                tracing::error!("Failed to update stock: {:?}", e);
                Error::internal("Stock update failed".to_string())
            })?;

            let price = product.price;
            let quantity_decimal = Decimal::from(item.quantity);
            total_amount += price * quantity_decimal;

            // Create Order Item
            let order_item = order_item::ActiveModel {
                order_id: Set(order_model.id),
                product_id: Set(item.product_id),
                quantity: Set(item.quantity),
                price_per_unit: Set(price),
                ..Default::default()
            };

            order_item.insert(&txn).await.map_err(|e| {
                tracing::error!("Failed to create order item: {:?}", e);
                Error::internal("Order item creation failed".to_string())
            })?;
        }

        // 3. Update Order Total
        let mut order_active: order::ActiveModel = order_model.into();
        order_active.total_amount = Set(total_amount);
        let order_model = order_active.update(&txn).await.map_err(|e| {
            tracing::error!("Failed to update order total: {:?}", e);
            Error::internal("Order update failed".to_string())
        })?;

        txn.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {:?}", e);
            Error::internal("Transaction commit failed".to_string())
        })?;

        Ok(order_model)
    }
}
