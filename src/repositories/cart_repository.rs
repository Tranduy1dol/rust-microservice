use std::collections::HashMap;

use chrono::Utc;
use entities::{
    cart::{ActiveModel as CartActiveModel, Entity as Cart, Model as CartModel},
    cart_item::{
        ActiveModel as CartItemActiveModel, Column as CartItemColumn, Entity as CartItem,
        Model as CartItemModel,
    },
    product::{Entity as Product, Model as ProductModel},
};
use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, NotSet, QueryFilter,
};

use crate::errors::Error;

pub struct CartRepository {
    database: DatabaseConnection,
}

impl CartRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn create_cart(&self, user_id: i64) -> Result<CartModel, Error> {
        let now = Utc::now().timestamp_millis();
        let active_model = CartActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        active_model
            .insert(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn get_cart_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Option<(CartModel, Vec<(CartItemModel, ProductModel)>)>, Error> {
        let cart_with_items: Option<(CartModel, Vec<CartItemModel>)> = Cart::find()
            .filter(entities::cart::Column::UserId.eq(user_id))
            .find_with_related(CartItem)
            .all(&self.database)
            .await
            .map_err(Error::from)?
            .into_iter()
            .next();

        let (cart, items) = match cart_with_items {
            Some((cart, items)) => (cart, items),
            None => return Ok(None),
        };

        if items.is_empty() {
            return Ok(Some((cart, vec![])));
        }

        let product_ids: Vec<i64> = items.iter().map(|item| item.product_id).collect();
        let products: Vec<ProductModel> = Product::find()
            .filter(entities::product::Column::Id.is_in(product_ids))
            .all(&self.database)
            .await
            .map_err(Error::from)?;

        let products_map: HashMap<i64, ProductModel> =
            products.into_iter().map(|p| (p.id, p)).collect();

        let items_with_products: Vec<(CartItemModel, ProductModel)> = items
            .into_iter()
            .filter_map(|item| {
                products_map
                    .get(&item.product_id)
                    .cloned()
                    .map(|product| (item, product))
            })
            .collect();

        Ok(Some((cart, items_with_products)))
    }

    pub async fn set_item_quantity(
        &self,
        user_id: i64,
        product_id: i64,
        quantity: i32,
    ) -> Result<CartItemModel, Error> {
        let cart = Cart::find()
            .filter(entities::cart::Column::UserId.eq(user_id))
            .one(&self.database)
            .await?
            .ok_or(Error::not_found("Cart not found".to_string()))?;

        let active_model = CartItemActiveModel {
            id: NotSet,
            cart_id: Set(cart.id as i64),
            product_id: Set(product_id),
            quantity: Set(quantity),
        };

        CartItem::insert(active_model)
            .on_conflict(
                OnConflict::columns([CartItemColumn::CartId, CartItemColumn::ProductId])
                    .update_column(CartItemColumn::Quantity)
                    .to_owned(),
            )
            .exec_with_returning(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn remove_item(&self, user_id: i64, product_id: i64) -> Result<u64, Error> {
        let cart = Cart::find()
            .filter(entities::cart::Column::UserId.eq(user_id))
            .one(&self.database)
            .await?
            .ok_or(Error::not_found("Cart not found".to_string()))?;

        let res = CartItem::delete_many()
            .filter(CartItemColumn::CartId.eq(cart.id as i64))
            .filter(CartItemColumn::ProductId.eq(product_id))
            .exec(&self.database)
            .await?;

        Ok(res.rows_affected)
    }
}
