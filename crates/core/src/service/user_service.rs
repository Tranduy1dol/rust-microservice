use std::sync::Arc;

use entities::user;
use validator::Validate;

use crate::{
    dto::user_dto::{LoginDto, RegisterUserDto},
    error::Error,
    ports::user_repo::UserRepository,
};

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, dto: RegisterUserDto) -> Result<user::Model, Error> {
        dto.validate()?;

        let password_hash = bcrypt::hash(dto.password, bcrypt::DEFAULT_COST)?;

        self.user_repo
            .create_new(
                dto.user_name,
                dto.email,
                password_hash,
                dto.first_name,
                dto.last_name,
                dto.address,
            )
            .await
    }

    pub async fn login(&self, dto: LoginDto) -> Result<String, Error> {
        let user = self.user_repo.get_by_email(dto.email).await?;

        let valid = bcrypt::verify(dto.password, &user.password_hash)?;

        if !valid {
            return Err(Error::unauthorized("Invalid credentials".to_string()));
        }

        let token = "temp_jwt_token".to_string();

        Ok(token)
    }
}
