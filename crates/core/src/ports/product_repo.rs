use async_trait::async_trait;
use entities::product::Model as ProductModel;
use rust_decimal::Decimal;

use crate::error::Error;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create_new(
        &self,
        name: String,
        description: Option<String>,
        price: Decimal,
        category_id: Option<i64>,
        stock_quantity: i32,
    ) -> Result<ProductModel, Error>;

    async fn get_by_id(&self, id: i64) -> Result<ProductModel, Error>;

    async fn get_by_category_id(
        &self,
        category_id: i64,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ProductModel>, Error>;

    async fn get_all(&self) -> Result<Vec<ProductModel>, Error>;

    async fn get_for_update(&self, id: i64) -> Result<ProductModel, Error>;

    async fn search_by_query_string(
        &self,
        query_string: String,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ProductModel>, Error>;

    async fn update_stock(&self, id: i64, new_quantity: i32) -> Result<ProductModel, Error>;

    async fn update_detail_by_id(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        price: Decimal,
        category_id: Option<i64>,
    ) -> Result<ProductModel, Error>;

    async fn delete_by_id(&self, id: i64) -> Result<ProductModel, Error>;
}
