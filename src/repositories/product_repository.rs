use chrono::Utc;
use entities::product::{
    ActiveModel as ProductActiveModel, Column, Entity as Product, Model as ProductModel,
};
use rust_decimal::Decimal;
use sea_orm::QueryFilter;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, NotSet, PaginatorTrait, QueryOrder, Set,
};

use crate::errors::Error;

pub struct ProductRepository {
    database: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn create_new_product(
        &self,
        name: String,
        description: Option<String>,
        price: Decimal,
        category_id: Option<i64>,
        stock_quantity: i32,
    ) -> Result<ProductModel, Error> {
        let now = Utc::now().timestamp_millis();

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
        let mut active_model = ProductActiveModel {
            id: Set(id),
            description: Set(description),
            category_id: Set(category_id),
            updated_at: Set(Utc::now().timestamp_millis()),
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

    pub async fn get_product_by_id(&self, id: i64) -> Result<ProductModel, Error> {
        Product::find_by_id(id)
            .one(&self.database)
            .await
            .map_err(Error::from)?
            .ok_or(Error::not_found(format!("Product {} not found", id)))
    }

    pub async fn get_all_products(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<ProductModel>, u64), Error> {
        let paginator = Product::find()
            .order_by_asc(Column::Name)
            .paginate(&self.database, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }

    pub async fn search_products(
        &self,
        query: String,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<ProductModel>, u64), Error> {
        let paginator = Product::find()
            .filter(Column::Name.contains(&query))
            .order_by_asc(Column::Name)
            .paginate(&self.database, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }

    pub async fn get_products_by_category(
        &self,
        category_id: i64,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<ProductModel>, u64), Error> {
        let paginator = Product::find()
            .filter(Column::CategoryId.eq(category_id))
            .order_by_asc(Column::Name)
            .paginate(&self.database, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }
}
