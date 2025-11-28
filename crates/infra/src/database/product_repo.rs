use app_core::{error::Error, ports::product_repo::ProductRepository};
use async_trait::async_trait;
use chrono::Utc;
use entities::product;
use sea_orm::prelude::*;
use sea_orm::{NotSet, QueryOrder, Set};

#[derive(Clone)]
pub struct SeaOrmProductRepo {
    db: DatabaseConnection,
}

impl SeaOrmProductRepo {
    /// Creates a SeaOrmProductRepo bound to the provided database connection.
    ///
    /// Returns the constructed repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sea_orm::DatabaseConnection;
    /// use crate::repos::SeaOrmProductRepo;
    ///
    /// // obtain a DatabaseConnection from your application setup
    /// let db: DatabaseConnection = /* ... */ unimplemented!();
    /// let repo = SeaOrmProductRepo::new(db);
    /// ```
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProductRepository for SeaOrmProductRepo {
    /// Creates a new product record and inserts it into the database.
    ///
    /// The product's `created_at` and `updated_at` timestamps are set to the current UTC time in milliseconds.
    ///
    /// # Returns
    ///
    /// `product::Model` representing the inserted product.
    ///
    /// # Examples
    ///
    /// ```
    /// // assumes `repo: SeaOrmProductRepo` and `price: rust_decimal::Decimal` are available
    /// # async fn example(repo: &SeaOrmProductRepo, price: rust_decimal::Decimal) {
    /// let product = repo.create_new("Soda".into(), None, price, None, 10).await.unwrap();
    /// assert_eq!(product.name, "Soda");
    /// # }
    /// ```
    async fn create_new(
        &self,
        name: String,
        description: Option<String>,
        price: Decimal,
        category_id: Option<i64>,
        stock_quantity: i32,
    ) -> Result<product::Model, Error> {
        let now = Utc::now().timestamp_millis();

        let active_model = product::ActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
            price: Set(price),
            category_id: Set(category_id),
            stock_quantity: Set(stock_quantity),
            created_at: Set(now),
            updated_at: Set(now),
        };

        product::Entity::insert(active_model)
            .exec_with_returning(&self.db)
            .await
            .map_err(Error::from)
    }

    /// Retrieves a product by its numeric ID.
    ///
    /// # Returns
    ///
    /// `Ok` with the matching `product::Model` if a product with the given ID exists, `Err` with a repository `Error` when the product is not found or a database error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::repositories::seaorm::SeaOrmProductRepo;
    /// # async fn example(repo: &SeaOrmProductRepo) {
    /// let product = repo.get_by_id(1).await.unwrap();
    /// assert_eq!(product.id, 1);
    /// println!("Found product: {}", product.name);
    /// # }
    /// ```
    async fn get_by_id(&self, id: i64) -> Result<product::Model, Error> {
        product::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(Error::from)?
            .ok_or(Error::not_found(format!("Product {} not found", id)))
    }

    /// Fetches a single page of products for the specified category, ordered by name.
    ///
    /// # Returns
    ///
    /// A tuple `(Vec<product::Model>, u64)` where the first element is the list of products for the requested page and the second element is the total number of pages available.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(repo: &SeaOrmProductRepo) -> Result<(), Error> {
    /// let (products, total_pages) = repo.get_by_category_id(42, 1, 20).await?;
    /// assert!(total_pages >= 1);
    /// # Ok(()) }
    /// ```
    async fn get_by_category_id(
        &self,
        category_id: i64,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        let paginator = product::Entity::find()
            .filter(product::Column::CategoryId.eq(category_id))
            .order_by_asc(product::Column::Name)
            .paginate(&self.db, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }

    /// Retrieve a single page of all products ordered by name (ascending).
    ///
    /// Page numbering is 1-based; `page_size` controls the number of items per page.
    ///
    —
    /// # Returns
    ///
    /// A tuple where the first element is a vector of products for the requested page, and the second is the total number of pages.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::repos::SeaOrmProductRepo;
    /// # async fn example(repo: &SeaOrmProductRepo) {
    /// let (products, total_pages) = repo.get_all(1, 20).await.unwrap();
    /// assert!(total_pages >= 1);
    /// let _first_page: Vec<_> = products;
    /// # }
    /// ```
    async fn get_all(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        let paginator = product::Entity::find()
            .order_by_asc(product::Column::Name)
            .paginate(&self.db, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }

    /// Searches products whose name contains the given query string and returns the requested page along with the total number of pages.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use crate::repos::SeaOrmProductRepo;
    /// # async fn example(repo: &SeaOrmProductRepo) {
    /// let (products, total_pages) = repo.search_by_query_string("phone".into(), 1, 10).await.unwrap();
    /// assert!(total_pages >= 0);
    /// # }
    /// ```ignore
    ///
    /// # Returns
    ///
    /// A tuple where the first element is a vector of products for the requested page and the second element is the total number of pages (`u64`).
    async fn search_by_query_string(
        &self,
        query_string: String,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<product::Model>, u64), Error> {
        let paginator = product::Entity::find()
            .filter(product::Column::Name.contains(&query_string))
            .order_by_asc(product::Column::Name)
            .paginate(&self.db, page_size);

        let num_pages = paginator.num_pages().await.map_err(Error::from)?;
        let products = paginator.fetch_page(page - 1).await.map_err(Error::from)?;

        Ok((products, num_pages))
    }

    /// Update a product's stock quantity and set its `updated_at` timestamp to the current time.
    ///
    /// Attempts to persist `new_quantity` to the product identified by `id`. On success returns the
    /// updated `product::Model`; on failure returns a repository `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a runtime and repository (pseudo-code — replace with your setup)
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    /// let repo = /* SeaOrmProductRepo::new(db_conn) */;
    ///
    /// let updated = rt.block_on(async {
    ///     repo.update_stock(42, 10).await.unwrap()
    /// });
    ///
    /// assert_eq!(updated.id, 42);
    /// assert_eq!(updated.stock_quantity, 10);
    /// ```
    async fn update_stock(&self, id: i64, new_quantity: i32) -> Result<product::Model, Error> {
        let active_model = product::ActiveModel {
            id: Set(id),
            updated_at: Set(Utc::now().timestamp_millis()),
            stock_quantity: Set(new_quantity),
            ..Default::default()
        };

        product::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(Error::from)
    }

    /// Update the specified fields of the product with the given id.
    ///
    /// Only the optional fields provided (`name`, `description`, `price`, `category_id`) are changed; `updated_at` is set to the current UTC timestamp in milliseconds. Fields passed as `None` are left unchanged.
    ///
    /// # Returns
    ///
    /// The updated `product::Model` on success.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Assumes `repo` is a `SeaOrmProductRepo` and `Decimal` is in scope.
    /// let updated = repo
    ///     .update_detail_by_id(
    ///         42,
    ///         Some("New name".to_string()),
    ///         Some("Updated description".to_string()),
    ///         Some(Decimal::new(1999, 2)), // 19.99
    ///         Some(3),
    ///     )
    ///     .await
    ///     .unwrap();
    /// assert_eq!(updated.id, 42);
    /// ```
    async fn update_detail_by_id(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        price: Option<Decimal>,
        category_id: Option<i64>,
    ) -> Result<product::Model, Error> {
        let mut active_model = product::ActiveModel {
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

        product::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map_err(Error::from)
    }

    /// Deletes the product with the specified id and returns the deleted model.
    ///
    /// If no product with the specified id exists, returns an `Error::not_found`.
    ///
    /// # Returns
    ///
    /// `Ok(product::Model)` containing the deleted product, `Err(Error::not_found(_))` if no such product exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(repo: &impl crate::repositories::ProductRepository) {
    /// let deleted = repo.delete_by_id(1).await.unwrap();
    /// assert_eq!(deleted.id, 1);
    /// # }
    /// ```
    async fn delete_by_id(&self, id: i64) -> Result<product::Model, Error> {
        let results = product::Entity::delete_by_id(id)
            .exec_with_returning(&self.db)
            .await?;

        results
            .into_iter()
            .next()
            .ok_or(Error::not_found(format!("No such product with id {}", id)))
    }
}