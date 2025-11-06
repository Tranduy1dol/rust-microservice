use entities::category::{
    ActiveModel as CategoryActiveModel, Entity as Category, Model as CategoryModel,
};
use sea_orm::{DatabaseConnection, EntityTrait, NotSet, Set};

use crate::errors::Error;

pub struct CategoryRepository {
    database: DatabaseConnection,
}

impl CategoryRepository {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn create_new_category(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<CategoryModel, Error> {
        let active_model = CategoryActiveModel {
            id: NotSet,
            name: Set(name),
            description: Set(description),
        };

        Category::insert(active_model)
            .exec_with_returning(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn delete_category(&self, id: i64) -> Result<CategoryModel, Error> {
        let models = Category::delete_by_id(id)
            .exec_with_returning(&self.database)
            .await
            .map_err(Error::from)?;

        models
            .into_iter()
            .next()
            .ok_or(Error::not_found(format!("Category {} not found", id)))
    }

    pub async fn update_category(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<CategoryModel, Error> {
        let mut active_model = CategoryActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(name) = name {
            active_model.name = Set(name);
        }

        if let Some(description) = description {
            active_model.description = Set(Some(description));
        }

        Category::update(active_model)
            .exec(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn get_category(&self, id: i64) -> Result<CategoryModel, Error> {
        Category::find_by_id(id)
            .one(&self.database)
            .await
            .map_err(Error::from)?
            .ok_or(Error::not_found(format!("Category {} not found", id)))
    }
}
