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
    /// Constructs a new ProductService that wraps the given product repository.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// let repo: Arc<dyn ProductRepository> = Arc::new(MyRepo::new());
    /// let service = ProductService::new(repo);
    /// ```
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    /// Create a new product from the provided DTO.
    ///
    /// Validates the DTO and persists a new product through the configured repository.
    ///
    /// # Returns
    ///
    /// `Ok(product::Model)` containing the created product, `Err(Error)` on failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn example() {
    /// let repo = /* implement ProductRepository */ unimplemented!();
    /// let svc = ProductService::new(std::sync::Arc::new(repo));
    /// let dto = CreateProductDto {
    ///     name: "Widget".into(),
    ///     category_id: 1,
    ///     price: 9.99,
    ///     description: Some("A useful widget".into()),
    ///     stock_quantity: 10,
    /// };
    /// let created = svc.create_new(dto).await.unwrap();
    /// assert_eq!(created.name, "Widget");
    /// # }
    /// ```
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

    /// Fetches the product with the specified identifier.
    ///
    /// # Returns
    ///
    /// `product::Model` when a product with the given `id` exists, `Error` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn run_example(service: &ProductService) {
    /// let product = service.get_by_id(1).await.unwrap();
    /// assert_eq!(product.id, 1);
    /// # }
    /// ```
    pub async fn get_by_id(&self, id: i64) -> Result<product::Model, Error> {
        self.product_repo.get_by_id(id).await
    }

    /// Fetches products for a specific category with pagination.
    ///
    /// Validates the provided `Pagination` and returns the products for the requested page together with the total number of matching products.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use crates_core::service::product_service::ProductService;
    /// # use crates_core::dto::Pagination;
    /// # async fn example(svc: &ProductService) -> Result<(), Box<dyn std::error::Error>> {
    /// let (products, total) = svc.get_by_category_id(42, Pagination { page: 1, page_size: 10 }).await?;
    /// assert!(total >= products.len() as u64);
    /// # Ok(()) }
    /// ```
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

    /// Fetches products for the given page and page size.
    ///
    /// Validates the provided `Pagination` and returns the products for that page along with the total number of products across all pages.
    ///
    /// # Returns
    ///
    /// A tuple `(Vec<product::Model>, u64)` where the first element is the list of products for the requested page and the second element is the total number of products.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # fn example() {
    /// #     tokio_test::block_on(async {
    /// let svc = todo!(); // ProductService instance
    /// let (products, total) = svc.get_all(Pagination { page: 1, page_size: 10 }).await.unwrap();
    /// assert!(total >= products.len() as u64);
    /// #     });
    /// # }
    /// ```
    pub async fn get_all(
        &self,
        pagination: Pagination,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        pagination.validate()?;
        let Pagination { page, page_size } = pagination;

        self.product_repo.get_all(page, page_size).await
    }

    /// Searches products by a free-text query and returns paginated matches.
    ///
    /// Validates the search query and pagination parameters, then returns the products
    /// matching the query for the requested page.
    ///
    /// # Returns
    ///
    /// A tuple containing a vector of matching `product::Model` and the total number of matches.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use crate::service::product_service::ProductService;
    /// # use crate::dto::{SearchQueryDto, Pagination};
    /// # async fn example(service: &ProductService) {
    /// let query = SearchQueryDto { q: "phone".into() };
    /// let pagination = Pagination { page: 1, page_size: 10 };
    /// let (products, total) = service.search(query, pagination).await.unwrap();
    /// assert!(total >= products.len() as u64);
    /// # }
    /// ```
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

    /// Update the stock quantity for a product by its id.
    ///
    /// If `new_quantity` is less than zero, returns a `bad_request` `Error` with the message
    /// "Stock quantity cannot be negative". On success, returns the updated `product::Model`.
    /// Repository errors are propagated.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use futures::executor::block_on;
    /// # // setup: service and repo would be created in real code
    /// # let repo = /* Arc<dyn ProductRepository> */ unimplemented!();
    /// # let service = /* ProductService::new(repo) */ unimplemented!();
    /// # let id = 1i64;
    /// # let new_qty = 10i32;
    /// let res = block_on(service.update_stock(id, new_qty));
    /// match res {
    ///     Ok(updated) => assert_eq!(updated.stock_quantity, new_qty),
    ///     Err(e) => panic!("update failed: {:?}", e),
    /// }
    /// ```
    pub async fn update_stock(&self, id: i64, new_quantity: i32) -> Result<product::Model, Error> {
        if new_quantity < 0 {
            return Err(Error::bad_request(
                "Stock quantity cannot be negative".to_string(),
            ));
        }

        self.product_repo.update_stock(id, new_quantity).await
    }

    /// Validate and apply updated product fields from `UpdateProductDto`.
    ///
    /// The DTO is validated; on success the repository is asked to update the product's
    /// name, description, price, and category and the updated model is returned.
    ///
    /// # Returns
    ///
    /// The updated `product::Model` on success.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// async fn example(service: &crate::service::product_service::ProductService) {
    ///     let dto = crate::dto::UpdateProductDto {
    ///         product_id: 1,
    ///         name: "New name".into(),
    ///         category_id: 2,
    ///         price: 19.99,
    ///         description: Some("Updated description".into()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let updated = service.update_detail_by_id(dto).await.unwrap();
    ///     assert_eq!(updated.id, 1);
    /// }
    /// ```
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

    /// Deletes a product by its ID.
    ///
    /// Returns the deleted `product::Model`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use crates::core::service::ProductService;
    /// # use crates::core::repository::ProductRepository;
    /// # async fn example(repo: Arc<dyn ProductRepository>) -> Result<(), Box<dyn std::error::Error>> {
    /// let svc = ProductService::new(repo);
    /// let deleted = svc.delete_by_id(42).await?;
    /// // `deleted` is the removed `product::Model`
    /// # Ok(()) }
    /// ```
    pub async fn delete_by_id(&self, id: i64) -> Result<product::Model, Error> {
        self.product_repo.delete_by_id(id).await
    }
}