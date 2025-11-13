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
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProductRepository for SeaOrmProductRepo {
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

    async fn get_by_id(&self, id: i64) -> Result<product::Model, Error> {
        product::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(Error::from)?
            .ok_or(Error::not_found(format!("Product {} not found", id)))
    }

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
