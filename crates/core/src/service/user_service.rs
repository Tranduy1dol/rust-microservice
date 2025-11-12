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
    /// Creates a new UserService that uses the provided user repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// // `repo` must implement `UserRepository`.
    /// // let repo: Arc<dyn UserRepository> = Arc::new(MyUserRepo::new());
    /// // let svc = UserService::new(repo);
    /// ```
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Register a new user from the provided registration data.
    ///
    /// Validates the `RegisterUserDto`, stores a hashed password, and returns the created user model.
    ///
    /// # Returns
    ///
    /// The created `user::Model`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use crates_core::service::UserService;
    /// # use crates_core::dto::RegisterUserDto;
    /// # use crates_core::repository::InMemoryUserRepo;
    /// // Construct service with a repository and call register.
    /// let repo = Arc::new(InMemoryUserRepo::default());
    /// let svc = UserService::new(repo);
    /// let dto = RegisterUserDto {
    ///     user_name: "alice".to_string(),
    ///     email: "alice@example.com".to_string(),
    ///     password: "s3cret".to_string(),
    ///     first_name: Some("Alice".to_string()),
    ///     last_name: Some("Example".to_string()),
    ///     address: None,
    /// };
    /// let created = tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     svc.register(dto).await.unwrap()
    /// });
    /// assert_eq!(created.user_name, "alice");
    /// ```
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

    /// Attempts to authenticate a user with the provided credentials and returns an authentication token on success.
    ///
    /// Returns an unauthorized `Error` when the email exists but the password does not match. Other errors from the user
    /// repository or password verification are propagated.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::executor::block_on;
    /// // let service = /* UserService instance */;
    /// // let dto = /* LoginDto with email and password */;
    /// // let token = block_on(service.login(dto)).unwrap();
    /// // assert_eq!(token, "temp_jwt_token".to_string());
    /// ```
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