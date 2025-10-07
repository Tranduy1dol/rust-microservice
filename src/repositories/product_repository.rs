use std::sync::LazyLock;

use chrono::Utc;
use entities::product::{
    ActiveModel as ProductActiveModel, Entity as Product, Model as ProductModel,
};
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, NotSet, Set};

use crate::database::get_database;
use crate::errors::Error;
use crate::repositories::category_repository::CATEGORY_REPOSITORY;

pub struct ProductRepository {
    database: DatabaseConnection,
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }

    pub async fn create_new_product(
        &self,
        name: String,
        description: Option<String>,
        price: Decimal,
        category_id: Option<i64>,
        stock_quantity: i32,
    ) -> Result<ProductModel, Error> {
        let now = Utc::now().timestamp();

        if category_id.is_some() {
            if let Err(err) = CATEGORY_REPOSITORY.get_category(category_id.unwrap()).await {
                return Err(Error::from(err));
            }
        }

        let active_model = ProductActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            price: Set(price),
            category_id: Set(category_id),
            stock_quantity: Set(stock_quantity),
            created_at: Set(now),
            updated_at: Set(now),
        };

        Product::insert(active_model)
            .exec_with_returning(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn delete_product_by_id(&self, id: i64) -> Result<ProductModel, Error> {
        let results = Product::delete_by_id(id)
            .exec_with_returning(&self.database)
            .await?;

        results
            .into_iter()
            .next()
            .ok_or(Error::not_found(format!("No such product with id {}", id)))
    }

    pub async fn update_product_detail_by_id(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        price: Option<Decimal>,
        category_id: Option<i64>,
        stock_quantity: Option<i32>,
    ) -> Result<ProductModel, Error> {
        if category_id.is_some() {
            if let Err(err) = CATEGORY_REPOSITORY.get_category(category_id.unwrap()).await {
                return Err(Error::from(err));
            }
        }

        let mut active_model = ProductActiveModel {
            id: Set(id),
            description: Set(description),
            category_id: Set(category_id),
            updated_at: Set(Utc::now().timestamp()),
            ..Default::default()
        };

        if let Some(name) = name {
            active_model.name = Set(name);
        }

        if let Some(price) = price {
            active_model.price = Set(price);
        }

        if let Some(stock_quantity) = stock_quantity {
            active_model.stock_quantity = Set(stock_quantity);
        }

        Product::update(active_model)
            .exec(&self.database)
            .await
            .map_err(Error::from)
    }
}

pub static PRODUCT_REPOSITORY: LazyLock<ProductRepository> = LazyLock::new(ProductRepository::new);
