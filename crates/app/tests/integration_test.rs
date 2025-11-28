use std::sync::Arc;

use app_core::dto::cart_dto::{AddCartItemDto, CartDto};
use app_core::service::{
    cart_service::CartService, checkout_service::CheckoutService, product_service::ProductService,
    user_service::UserService,
};
use app_lib::state::AppState;
use axum::Extension;
use infra::cache::RedisCartRepository;
use infra::database::{
    checkout_repo::SeaOrmCheckoutRepo, product_repo::SeaOrmProductRepo, user_repo::SeaOrmUserRepo,
};
use migration::{Migrator, MigratorTrait};
use reqwest::Client;
use sea_orm::{Database, DatabaseConnection};
use testcontainers::ContainerAsync;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::redis::Redis;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

struct TestApp {
    pub address: String,
    pub db_pool: DatabaseConnection,
    // Keep containers alive
    pub _postgres: ContainerAsync<Postgres>,
    pub _redis: ContainerAsync<Redis>,
}

async fn spawn_app() -> TestApp {
    // 1. Start Postgres
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

    // 2. Start Redis
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

    // 3. Run Migrations
    let db_pool = Database::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");
    Migrator::up(&db_pool, None)
        .await
        .expect("Failed to run migrations");

    // 4. Setup App State
    let redis_pool = infra::cache::create_connection_pool(&redis_url)
        .await
        .expect("Failed to connect to Redis");

    let (event_sender, _) = mpsc::channel(100);

    let user_repo = Arc::new(SeaOrmUserRepo::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(
        user_repo,
        "test_secret".to_string(),
        event_sender,
    ));

    let product_repo = Arc::new(SeaOrmProductRepo::new(db_pool.clone()));
    let product_service = Arc::new(ProductService::new(product_repo));

    let cart_repo = Arc::new(RedisCartRepository::new(redis_pool));
    let cart_service = Arc::new(CartService::new(cart_repo));

    let checkout_repo = Arc::new(SeaOrmCheckoutRepo::new(db_pool.clone()));
    let checkout_service = Arc::new(CheckoutService::new(cart_service.clone(), checkout_repo));

    let state = AppState {
        user_service,
        product_service,
        cart_service,
        checkout_service,
    };

    let app = app_lib::router::create_router(state).layer(Extension(1i64)); // Mock Auth: Inject user_id = 1

    // 5. Bind to random port
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
    }
}

#[tokio::test]
async fn test_cart_and_checkout_flow() {
    let app = spawn_app().await;
    let client = Client::new();

    use entities::{product, user};
    use rust_decimal_macros::dec;
    use sea_orm::{ActiveModelTrait, Set};

    // 0. Seed User
    let user = user::ActiveModel {
        id: Set(1),
        username: Set("testuser".to_string()),
        email: Set("test@example.com".to_string()),
        password_hash: Set("hash".to_string()),
        first_name: Set("Test".to_string()),
        last_name: Set("User".to_string()),
        address: Set("123 Test St".to_string()),
        created_at: Set(chrono::Utc::now().timestamp()),
        updated_at: Set(chrono::Utc::now().timestamp()),
        ..Default::default()
    };
    let user = user
        .insert(&app.db_pool)
        .await
        .expect("Failed to seed user");
    assert_eq!(user.id, 1, "Expected user ID 1");

    // 1. Seed Product
    let active_product = product::ActiveModel {
        name: Set("Test Product".to_string()),
        description: Set(Some("Desc".to_string())),
        price: Set(dec!(100.00)),
        stock_quantity: Set(10),
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

    // 2. Add Item to Cart
    let response = client
        .post(format!("{}/api/v1/cart/items", app.address))
        .json(&AddCartItemDto {
            product_id,
            quantity: 2,
        })
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    let cart: CartDto = response.json().await.expect("Failed to parse cart");
    assert_eq!(cart.items.len(), 1);
    assert_eq!(cart.items[0].product_id, product_id);
    assert_eq!(cart.items[0].quantity, 2);

    // 3. Checkout
    let response = client
        .post(format!("{}/api/v1/checkout", app.address))
        .send()
        .await
        .expect("Failed to execute checkout");

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        panic!("Checkout failed with status {}: {}", status, text);
    }
    let order_json: serde_json::Value = response.json().await.expect("Failed to parse order");
    let order_id = order_json.get("orderId").unwrap().as_i64().unwrap();

    // 4. Verify Order in DB
    use entities::order;
    use sea_orm::EntityTrait;

    let order = order::Entity::find_by_id(order_id)
        .one(&app.db_pool)
        .await
        .expect("Failed to fetch order")
        .expect("Order not found");

    assert_eq!(order.user_id, 1);
    assert_eq!(order.total_amount, dec!(200.00)); // 2 * 100.00

    // 5. Verify Stock Deducted
    let updated_product = product::Entity::find_by_id(product_id)
        .one(&app.db_pool)
        .await
        .expect("Failed to fetch product")
        .unwrap();

    assert_eq!(updated_product.stock_quantity, 8); // 10 - 2

    // 6. Verify Cart Cleared
    let response = client
        .get(format!("{}/api/v1/cart", app.address))
        .send()
        .await
        .expect("Failed to get cart");

    let cart: CartDto = response.json().await.expect("Failed to parse cart");
    assert!(cart.items.is_empty());
}
