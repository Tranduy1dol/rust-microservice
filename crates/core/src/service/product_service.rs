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
    /// Creates a new ProductService that uses the provided product repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// let repo: Arc<dyn ProductRepository> = Arc::new(MyRepo::new());
    /// let service = ProductService::new(repo);
    /// ```
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }

    /// Create a new product from the provided DTO.
    ///
    /// Attempts to validate the DTO and persist a new product record through the repository.
    ///
    /// # Returns
    ///
    /// `Ok(product::Model)` containing the created product on success, `Err(Error)` on failure.
    ///
    /// # Examples
    ///
    /// ```
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

    /// Fetches a product by its identifier.
    ///
    /// # Returns
    ///
    /// `Ok(product::Model)` containing the product when found, `Err(Error)` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run_example(service: &ProductService) {
    /// let product = service.get_by_id(1).await.unwrap();
    /// assert_eq!(product.id, 1);
    /// # }
    /// ```
    pub async fn get_by_id(&self, id: i64) -> Result<product::Model, Error> {
        self.product_repo.get_by_id(id).await
    }

    /// Fetches products belonging to a specific category using pagination.
    ///
    /// Validates the provided `Pagination` (returns an error if invalid) and returns the products for
    /// the requested page together with the total number of matching products.
    ///
    /// # Examples
    ///
    /// ```
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

    /// Retrieves a paginated list of all products.
    ///
    /// # Returns
    ///
    /// A tuple containing a vector of product models and the total number of products (`(Vec<product::Model>, u64)`).
    ///
    /// # Examples
    ///
    /// ```
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

    /// Searches products matching a free-text query and returns the paginated results.
    ///
    /// Validates the search query and pagination parameters, then returns the matching products
    /// for the requested page.
    ///
    /// # Returns
    ///
    /// A tuple containing a vector of matching `product::Model` and the total number of matches.
    ///
    /// # Examples
    ///
    /// ```
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

    /// Updates the stock quantity for the product with the given `id`.
    ///
    /// Returns the updated `product::Model` on success. If `new_quantity` is less than zero,
    /// returns a `bad_request` `Error`. Other failures from the repository are returned as `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use futures::executor::block_on;
    /// # // setup: service and repo would be created in real code
    /// # let repo = /* Arc<dyn ProductRepository> */ unimplemented!();
    /// # let service = /* ProductService::new(repo) */ unimplemented!();
    /// # let id = 1i64;
    /// # let new_qty = 10i32;
    /// // call (in async context)
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

    /// Updates a product's name, description, price, and category using values from the provided DTO.
    ///
    /// The DTO is validated before the repository update is performed.
    ///
    /// # Returns
    ///
    /// The updated `product::Model` on success.
    ///
    /// # Examples
    ///
    /// ```
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
    /// Returns the deleted product model on success.
    ///
    /// # Examples
    ///
    /// ```
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