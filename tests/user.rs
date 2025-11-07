use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use sea_orm::{ConnectionTrait, Statement};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use tower::ServiceExt;

use migration::MigratorTrait;
use shopping_cart::{
    app::{create_app, AppState},
    config::DbConfig,
    database::setup_database_connection,
    routes::user::dtos::{
        request::{LoginRequestDto, RegisterRequestDto, ResetPasswordRequestDto},
        response::{
            GetUserProfileResponseDto, LoginResponseDto, RegisterResponseDto,
            ResetPasswordResponseDto,
        },
    },
};

/// Helper: Khởi động container, chạy migration, và tạo App
/// Hàm này sẽ được gọi bởi *mỗi* bài test
async fn setup_app_for_test() -> axum::Router {
    let postgres_instance = Postgres::default().start().await.unwrap();

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{:?}/postgres",
        postgres_instance.get_host_port_ipv4(5432).await.unwrap()
    );

    let test_db_config = Some(DbConfig {
        enable_query_log: false, // Tắt log cho test
        max_connections: 50,
        min_connections: 1,
        connect_timeout: 10,
        acquire_timeout: 10,
        idle_timeout: 300,
        max_lifetime: 1800,
    });

    let db_conn = setup_database_connection(&connection_string, &test_db_config)
        .await
        .unwrap();

    migration::Migrator::up(&db_conn, None).await.unwrap();

    // TRUNCATE (dọn dẹp) được gọi ngay sau migration để đảm bảo DB sạch
    db_conn
        .execute(Statement::from_string(
            db_conn.get_database_backend(),
            r#"TRUNCATE TABLE
            "user", "admin", "category", "product",
            "cart", "cart_item", "order", "order_item", "review"
           CASCADE;"#
                .to_string(),
        ))
        .await
        .unwrap();

    // Giả định AppState::new() đã được bạn implement
    let test_app_state = AppState::new(&connection_string, &test_db_config)
        .await
        .unwrap();

    create_app(test_app_state).await
}

/// Helper: Tạo DTO đăng ký
fn a_register_request(email: &str, password: &str) -> RegisterRequestDto {
    RegisterRequestDto {
        user_name: "Test User".to_string(),
        email: email.to_string(),
        password: password.to_string(),
        confirm_password: password.to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        address: "123 Test St".to_string(),
    }
}

/// Helper: Tạo DTO đăng nhập
fn a_login_request(email: &str, password: &str) -> LoginRequestDto {
    LoginRequestDto {
        email: email.to_string(),
        password: password.to_string(),
    }
}

/// Helper: Đăng ký và đăng nhập user, trả về (app, token)
async fn setup_app_with_logged_in_user(email: &str, password: &str) -> (axum::Router, String) {
    let app = setup_app_for_test().await;

    // 1. Đăng ký
    let register_dto = a_register_request(email, password);
    let register_req = Request::builder()
        .uri("/user/register")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&register_dto).unwrap()))
        .unwrap();
    app.clone().oneshot(register_req).await.unwrap();

    // 2. Đăng nhập
    let login_dto = a_login_request(email, password);
    let login_req = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&login_dto).unwrap()))
        .unwrap();
    let response = app.clone().oneshot(login_req).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let login_json: LoginResponseDto = serde_json::from_slice(&body).unwrap();

    (app, login_json.token)
}

// --- CÁC BÀI TEST ---

#[tokio::test]
async fn test_register_success() {
    let test_app = setup_app_for_test().await; // Môi trường sạch

    let email = "test_user@example.com";
    let password = "ValidPassword123";
    let request_dto = a_register_request(email, password);

    let request = Request::builder()
        .uri("/user/register")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    let response = test_app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: RegisterResponseDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(body_json.email, email);
}

#[tokio::test]
async fn test_register_email_already_exists() {
    let test_app = setup_app_for_test().await;
    let email = "existing@example.com";
    let password = "Password123";
    let request_dto = a_register_request(email, password);

    // Lần 1: Đăng ký thành công
    let request1 = Request::builder()
        .uri("/user/register")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();
    let response1 = test_app.clone().oneshot(request1).await.unwrap();
    assert_eq!(response1.status(), StatusCode::OK);

    // Lần 2: Đăng ký trùng email
    let request2 = Request::builder()
        .uri("/user/register")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();
    let response2 = test_app.oneshot(request2).await.unwrap();

    // Lỗi DbErr::Custom trả về 500
    assert_eq!(response2.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_login_success() {
    let email = "login_success@example.com";
    let password = "Password123";

    // 1. Setup (đã bao gồm đăng ký user)
    let (test_app, _token) = setup_app_with_logged_in_user(email, password).await;

    // 2. Chuẩn bị Request
    let request_dto = a_login_request(email, password);
    let request = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: LoginResponseDto = serde_json::from_slice(&body).unwrap();
    assert!(!body_json.token.is_empty());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let email = "wrong_pass@example.com";
    let correct_password = "Password123";
    let wrong_password = "WrongPassword";

    // 1. Setup (đăng ký với mật khẩu đúng)
    let (test_app, _token) = setup_app_with_logged_in_user(email, correct_password).await;

    // 2. Chuẩn bị Request (dùng mật khẩu sai)
    let request_dto = a_login_request(email, wrong_password);
    let request = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_user_not_found() {
    let test_app = setup_app_for_test().await; // DB sạch, không user

    // 2. Chuẩn bị Request
    let request_dto = a_login_request("nobody@example.com", "any_password");
    let request = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra (Lỗi `get_user_by_email` trả về 500)
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_profile_success() {
    let email = "profile@example.com";
    let password = "Password123";

    // 1. Setup (đăng ký, đăng nhập và lấy token)
    let (test_app, token) = setup_app_with_logged_in_user(email, password).await;

    // 2. Chuẩn bị Request
    let request = Request::builder()
        .uri("/user/profile")
        .method(Method::GET)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: GetUserProfileResponseDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(body_json.email, email);
    assert_eq!(body_json.first_name, "Test");
}

#[tokio::test]
async fn test_profile_no_token() {
    let test_app = setup_app_for_test().await;

    // 2. Chuẩn bị Request (không có header AUTHORIZATION)
    let request = Request::builder()
        .uri("/user/profile")
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra (Auth extractor sẽ fail)
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_reset_password_success() {
    let email = "reset_pass@example.com";
    let old_password = "OldPassword123";
    let new_password = "NewPassword456";

    // 1. Setup (đăng ký, đăng nhập và lấy token)
    let (test_app, token) = setup_app_with_logged_in_user(email, old_password).await;

    // 2. Chuẩn bị Request
    let request_dto = ResetPasswordRequestDto {
        current_password: old_password.to_string(),
        new_password: new_password.to_string(),
        confirm_password: new_password.to_string(),
    };
    let request = Request::builder()
        .uri("/user/reset-password")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    // 3. Gọi API
    let response = test_app.clone().oneshot(request).await.unwrap();

    // 4. Kiểm tra
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: ResetPasswordResponseDto = serde_json::from_slice(&body).unwrap();
    assert_eq!(body_json.user_id, 1); // Giả sử là user đầu tiên

    // 5. Xác minh (Quan trọng): Thử đăng nhập bằng mật khẩu mới
    let login_dto_new = a_login_request(email, new_password);
    let login_req_new = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&login_dto_new).unwrap()))
        .unwrap();
    let login_res_new = test_app.clone().oneshot(login_req_new).await.unwrap();
    assert_eq!(login_res_new.status(), StatusCode::OK);

    // 6. Xác minh (Quan trọng): Thử đăng nhập bằng mật khẩu cũ
    let login_dto_old = a_login_request(email, old_password);
    let login_req_old = Request::builder()
        .uri("/user/login")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_string(&login_dto_old).unwrap()))
        .unwrap();
    let login_res_old = test_app.oneshot(login_req_old).await.unwrap();
    assert_eq!(login_res_old.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_reset_password_wrong_current_password() {
    let email = "reset_fail@example.com";
    let correct_password = "CorrectPassword123";
    let wrong_current_password = "WrongPassword";

    // 1. Setup
    let (test_app, token) = setup_app_with_logged_in_user(email, correct_password).await;

    // 2. Chuẩn bị Request
    let request_dto = ResetPasswordRequestDto {
        current_password: wrong_current_password.to_string(),
        new_password: "NewPassword456".to_string(),
        confirm_password: "NewPassword456".to_string(),
    };
    let request = Request::builder()
        .uri("/user/reset-password")
        .method(Method::POST)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&request_dto).unwrap()))
        .unwrap();

    // 3. Gọi API
    let response = test_app.oneshot(request).await.unwrap();

    // 4. Kiểm tra
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
