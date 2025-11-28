use std::sync::Arc;

use futures::future::join_all;
use infra::{
    cache::RedisCartRepository,
    database::{
        checkout_repo::SeaOrmCheckoutRepo, product_repo::SeaOrmProductRepo,
        user_repo::SeaOrmUserRepo,
    },
};
use migration::{Migrator, MigratorTrait};
use reqwest::Client;
use sea_orm::{Database, DatabaseConnection};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::{postgres::Postgres, redis::Redis};
use tokio::{net::TcpListener, sync::mpsc};

use app_core::{
    dto::cart_dto::AddCartItemDto,
    service::{
        auth_service::AuthService, cart_service::CartService, checkout_service::CheckoutService,
        product_service::ProductService, user_service::UserService,
    },
};
use app_lib::state::AppState;

struct TestApp {
    pub address: String,
    pub db_pool: DatabaseConnection,
    pub _postgres: ContainerAsync<Postgres>,
    pub _redis: ContainerAsync<Redis>,
    pub jwt_secret: String,
}

async fn spawn_app() -> TestApp {
    let postgres_node = Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres");
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        postgres_node
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port")
    );

    let redis_node = Redis::default()
        .start()
        .await
        .expect("Failed to start Redis");
    let redis_url = format!(
        "redis://127.0.0.1:{}",
        redis_node
            .get_host_port_ipv4(6379)
            .await
            .expect("Failed to get port")
    );

    let db_pool = Database::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");
    Migrator::up(&db_pool, None)
        .await
        .expect("Failed to run migrations");

    let redis_pool = infra::cache::create_connection_pool(&redis_url)
        .await
        .expect("Failed to connect to Redis");

    let (event_sender, _) = mpsc::channel(100);
    let jwt_secret = "test_secret".to_string();

    let user_repo = Arc::new(SeaOrmUserRepo::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(
        user_repo,
        jwt_secret.clone(),
        event_sender,
    ));

    let auth_service = Arc::new(AuthService::new(jwt_secret.clone()));

    let product_repo = Arc::new(SeaOrmProductRepo::new(db_pool.clone()));
    let product_service = Arc::new(ProductService::new(product_repo));

    let cart_repo = Arc::new(RedisCartRepository::new(redis_pool));
    let cart_service = Arc::new(CartService::new(cart_repo));

    let checkout_repo = Arc::new(SeaOrmCheckoutRepo::new(db_pool.clone()));
    let checkout_service = Arc::new(CheckoutService::new(cart_service.clone(), checkout_repo));

    let state = AppState {
        auth_service,
        user_service,
        product_service,
        cart_service,
        checkout_service,
    };

    let app = app_lib::router::create_router(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestApp {
        address,
        db_pool,
        _postgres: postgres_node,
        _redis: redis_node,
        jwt_secret,
    }
}

fn generate_test_token(user_id: i64, secret: &str) -> String {
    use app_core::dto::auth::TokenClaims;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{EncodingKey, Header, encode};

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::hours(1)).timestamp() as usize;

    let claims = TokenClaims {
        sub: user_id,
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

#[tokio::test]
async fn test_concurrent_checkout_race_condition() {
    let app = spawn_app().await;
    let client = Client::new();

    use entities::{product, user};
    use rust_decimal_macros::dec;
    use sea_orm::{ActiveModelTrait, Set};

    // 1. Seed Product with Low Stock (e.g., 1)
    let active_product = product::ActiveModel {
        name: Set("Hot Item".to_string()),
        description: Set(Some("Limited Edition".to_string())),
        price: Set(dec!(100.00)),
        stock_quantity: Set(1), // Only 1 in stock!
        category_id: Set(None),
        created_at: Set(chrono::Utc::now().timestamp()),
        updated_at: Set(chrono::Utc::now().timestamp()),
        ..Default::default()
    };
    let product = active_product
        .insert(&app.db_pool)
        .await
        .expect("Failed to seed product");
    let product_id = product.id;

    // 2. Prepare Concurrent Users
    let num_users = 5;
    let mut user_tokens = Vec::new();

    for i in 1..=num_users {
        let user = user::ActiveModel {
            id: Set(i as i64),
            username: Set(format!("user{}", i)),
            email: Set(format!("user{}@example.com", i)),
            password_hash: Set("hash".to_string()),
            first_name: Set("Test".to_string()),
            last_name: Set("User".to_string()),
            address: Set("Address".to_string()),
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..Default::default()
        };
        let user = user
            .insert(&app.db_pool)
            .await
            .expect("Failed to seed user");

        let token = generate_test_token(user.id, &app.jwt_secret);
        user_tokens.push(token.clone());

        // Add item to cart for each user
        let response = client
            .post(format!("{}/api/v1/cart/items", app.address))
            .header("Authorization", format!("Bearer {}", token))
            .json(&AddCartItemDto {
                product_id,
                quantity: 1,
            })
            .send()
            .await
            .expect("Failed to add to cart");
        assert!(response.status().is_success());
    }

    // 3. Execute Concurrent Checkouts
    let mut handles = Vec::new();
    for token in user_tokens {
        let client = client.clone();
        let address = app.address.clone();
        handles.push(tokio::spawn(async move {
            client
                .post(format!("{}/api/v1/checkout", address))
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
        }));
    }

    let results = join_all(handles).await;

    // 4. Verify Results
    let mut success_count = 0;
    let mut failure_count = 0;

    for res in results {
        let response = res.unwrap().unwrap();
        if response.status().is_success() {
            success_count += 1;
        } else {
            failure_count += 1;
        }
    }

    println!("Success: {}, Failure: {}", success_count, failure_count);

    // Only 1 should succeed
    assert_eq!(success_count, 1, "Expected exactly 1 successful checkout");
    assert_eq!(
        failure_count,
        num_users - 1,
        "Expected remaining checkouts to fail"
    );

    // 5. Verify Final Stock
    use sea_orm::EntityTrait;
    let updated_product = product::Entity::find_by_id(product_id)
        .one(&app.db_pool)
        .await
        .expect("Failed to fetch product")
        .unwrap();

    assert_eq!(updated_product.stock_quantity, 0, "Stock should be 0");
}
