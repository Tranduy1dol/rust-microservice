# Shopping Cart Microservice

[![CI](https://github.com/Tranduy1dol/shopping-cart/actions/workflows/ci.yml/badge.svg)](https://github.com/Tranduy1dol/shopping-cart/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Tranduy1dol/shopping-cart/branch/develop/graph/badge.svg?token=QLN1P3LEH2)](https://codecov.io/gh/Tranduy1dol/shopping-cart)
![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/Tranduy1dol/shopping-cart?utm_source=oss&utm_medium=github&utm_campaign=Tranduy1dol%2Fshopping-cart&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)

A robust, high-performance shopping cart microservice built with Rust, following Clean Architecture principles.

## 🚀 Tech Stack

- **Language:** Rust 2024
- **Web Framework:** Axum
- **Database:** PostgreSQL (Persistence), Redis (Caching/Cart)
- **ORM:** SeaORM
- **Architecture:** Clean Architecture (Hexagonal)
- **Containerization:** Docker & Docker Compose
- **Testing:** Testcontainers, Mockall

## ✨ Features

- **User Management**: Registration, Login (JWT), Profile.
- **Product Catalog**: Browse products, manage inventory (Admin).
- **Shopping Cart**: High-performance Redis-backed cart with persistence.
- **Checkout**: Transactional order creation with stock validation.
- **Reviews**: Product reviews and ratings.

## 📂 Project Structure

The project is organized as a Cargo Workspace with multiple crates to enforce separation of concerns:

- `crates/app`: Application entry point, HTTP handlers, dependency injection (wiring).
- `crates/core`: Domain logic, service interfaces (ports), DTOs. Pure Rust, no external infrastructure dependencies.
- `crates/infra`: Infrastructure implementation (Database adapters, Redis cache, external APIs).
- `crates/entities`: SeaORM entity definitions (Database schema).
- `crates/migration`: Database migrations.

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Latest Stable)
- [Docker](https://www.docker.com/) & Docker Compose

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Tranduy1dol/shopping-cart.git
   cd shopping-cart
   ```

2. **Start Infrastructure**
   Start PostgreSQL and Redis containers:
   ```bash
   docker-compose up -d
   ```

3. **Run Migrations**
   ```bash
   cargo run -p migration -- up
   ```

4. **Run the Application**
   ```bash
   cargo run -p app
   ```
   The server will start at `http://127.0.0.1:3000`.

## ⚙️ Configuration

Configuration is managed via `config/default.toml` and environment variables.

| Variable | Description | Default |
|----------|-------------|---------|
| `APP_SERVER__PORT` | HTTP Server Port | `3000` |
| `APP_DATABASE__URL` | PostgreSQL Connection String | `postgres://postgres:postgres@localhost:5432/shopping_cart` |
| `APP_REDIS__URL` | Redis Connection String | `redis://localhost:6379` |

## 🧪 Testing

Run unit and integration tests:

```bash
# Run all tests
cargo test --workspace

# Run integration tests (requires Docker)
cargo test --test integration_test

# Run race condition tests
cargo test --test race_condition_test
```

## 📚 API Documentation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/users/register` | Register a new user |
| `POST` | `/api/v1/users/login` | Login and get JWT |
| `GET` | `/api/v1/products` | List all products |
| `GET` | `/api/v1/cart` | Get current user's cart |
| `POST` | `/api/v1/cart/items` | Add item to cart |
| `DELETE` | `/api/v1/cart/items/{id}` | Remove item from cart |
| `POST` | `/api/v1/checkout` | Checkout and create order |

## 🚀 Performance
 
We use [k6](https://k6.io/) for stress testing.
 
**Scenario:**
- **Endpoint:** `GET /api/v1/products/all`
- **VUs (Virtual Users):** 50
- **Duration:** 2 minutes
 
**Results:**
- **Throughput:** ~8,074 RPS
- **Avg Latency:** 4.51ms
- **P95 Latency:** 7.61ms
 
## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
