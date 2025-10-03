use std::sync::LazyLock;

use chrono::Utc;
use entities::admin::{
    ActiveModel as AdminActiveModel, Column, Entity as Admin, Model as AdminModel, Model,
};
use sea_orm::{
    prelude::Expr,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

use crate::{database::get_database, errors::Error};

pub struct AdminRepository {
    database: DatabaseConnection,
}

impl Default for AdminRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl AdminRepository {
    pub fn new() -> Self {
        Self {
            database: get_database().to_owned(),
        }
    }

    pub async fn create_new_admin(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<AdminModel, Error> {
        if Admin::find()
            .filter(Column::Email.eq(email.clone()))
            .one(&self.database)
            .await
            .is_ok()
        {
            return Err(Error::from(DbErr::Custom(
                "Admin email already exists!".to_string(),
            )));
        }

        let password_hash = bcrypt::hash(password.as_bytes(), bcrypt::DEFAULT_COST)?;
        let active_model = AdminActiveModel {
            id: NotSet,
            name: Set(name),
            email: Set(email.to_string()),
            password_hash: Set(password_hash),
            level: Set(1),
            created_at: Set(Utc::now().timestamp_millis()),
            is_active: Set(true),
        };

        Admin::insert(active_model)
            .exec_with_returning(&self.database)
            .await
            .map_err(Error::from)
    }

    pub async fn update_admin_level(&self, email: String, level: i32) -> Result<AdminModel, Error> {
        let models = Admin::update_many()
            .col_expr(Column::Level, Expr::value(level))
            .filter(Column::Email.eq(email.clone()))
            .exec_with_returning(&self.database)
            .await?;

        models
            .into_iter()
            .next()
            .ok_or(Error::from(DbErr::Custom(format!(
                "Admin with email {email} not found"
            ))))
    }

    pub async fn get_admin_by_email(&self, email: String) -> Result<Model, Error> {
        Admin::find()
            .filter(Column::Email.eq(email.clone()))
            .one(&self.database)
            .await
            .map_err(Error::from)?
            .ok_or_else(|| {
                Error::from(DbErr::Custom(format!(
                    "Admin with email {} not found!",
                    email
                )))
            })
    }
}

pub static ADMIN_REPOSITORY: LazyLock<AdminRepository> = LazyLock::new(AdminRepository::new);
