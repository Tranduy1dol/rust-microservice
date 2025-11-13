use std::sync::Arc;

use entities::product;
use validator::Validate;

use crate::dto::product_dto::{CreateProductDto, SearchQueryDto, UpdateProductDto};
use crate::error::Error;
use crate::pagination::Pagination;
use crate::ports::product_repo::ProductRepository;

pub struct ProductService {
    product_repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    pub async fn create_new(&self, dto: CreateProductDto) -> Result<product::Model, Error> {
        dto.validate()?;
        let CreateProductDto {
            name,
            category_id,
            price,
            description,
            stock_quantity,
        } = dto;

        self.product_repo
            .create_new(name, description, price, category_id, stock_quantity)
            .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<product::Model, Error> {
        self.product_repo.get_by_id(id).await
    }

    pub async fn get_by_category_id(
        &self,
        category_id: i64,
        pagination: Pagination,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        pagination.validate()?;
        let Pagination { page, page_size } = pagination;

        self.product_repo
            .get_by_category_id(category_id, page, page_size)
            .await
    }

    pub async fn get_all(
        &self,
        pagination: Pagination,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        pagination.validate()?;
        let Pagination { page, page_size } = pagination;

        self.product_repo.get_all(page, page_size).await
    }

    pub async fn search(
        &self,
        query_string: SearchQueryDto,
        pagination: Pagination,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        query_string.validate()?;
        pagination.validate()?;
        let Pagination { page, page_size } = pagination;

        self.product_repo
            .search_by_query_string(query_string.q, page, page_size)
            .await
    }

    pub async fn update_stock(&self, id: i64, new_quantity: i32) -> Result<product::Model, Error> {
        self.product_repo.update_stock(id, new_quantity).await
    }

    pub async fn update_detail_by_id(
        &self,
        dto: UpdateProductDto,
    ) -> Result<product::Model, Error> {
        dto.validate()?;
        let UpdateProductDto {
            product_id,
            name,
            category_id,
            price,
            description,
            ..
        } = dto;

        self.product_repo
            .update_detail_by_id(product_id, name, description, price, category_id)
            .await
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<product::Model, Error> {
        self.product_repo.delete_by_id(id).await
    }
}
