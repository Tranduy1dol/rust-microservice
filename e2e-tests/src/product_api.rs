use anyhow::Result;
use serde::Deserialize;
use reqwest::Client;

// Định nghĩa lại DTO response để kiểm tra
#[derive(Deserialize, Debug)]
struct ProductResponseDto {
    id: i64,
    name: String,
    price: String, // reqwest thường đọc decimal là string
}

const API_BASE_URL: &str = "http://127.0.0.1:8080"; // Đảm bảo server đang chạy ở port 8080

#[tokio::test]
async fn test_get_all_products_public() -> Result<()> {
    let client = Client::new();

    // Giả sử server đang chạy
    let res = client
        .get(format!("{}/product", API_BASE_URL))
        .send()
        .await?;

    assert!(res.status().is_success());

    let products: Vec<ProductResponseDto> = res.json().await?;
    println!("Fetched products: {:?}", products);

    Ok(())
}

#[tokio::test]
async fn test_get_product_by_id_public() -> Result<()> {
    let client = Client::new();

    // Giả sử bạn có sản phẩm với ID = 1 (bạn cần tạo trước trong DB)
    let res = client
        .get(format!("{}/product/1", API_BASE_URL))
        .send()
        .await?;

    assert!(res.status().is_success());

    let product: ProductResponseDto = res.json().await?;
    assert_eq!(product.id, 1);
    println!("Fetched product: {:?}", product);

    Ok(())
}