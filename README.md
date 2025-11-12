# Shopping Cart Microservice
[![codecov](https://codecov.io/gh/tranduy1dol/rust-microservice/branch/develop/graph/badge.svg)](https://codecov.io/gh/tranduy1dol/rust-microservice)


## Description

---

## Structure

---

```
shopping_cart/
├── .github/
│   └── workflows/
│       └── ci.yml
│
├── config/
│   ├── default.toml
│   └── production.toml
│
├── crates/
│   │
│   ├── app/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── state.rs
│   │   │   ├── router.rs
│   │   │   ├── handlers/
│   │   │   ├── worker.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── middleware/
│   │   └── Cargo.toml
│   │
│   ├── core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── events.rs
│   │   │   ├── ports/
│   │   │   └── services/
│   │   └── Cargo.toml
│   │
│   ├── infrastructure/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── database/
│   │   │   └── cache/
│   │   └── Cargo.toml
│   │
│   ├── entities/
│   └── migration/
│
├── tests/
│   ├── common.rs
│   ├── user_api.rs
│   ├── cart_api.rs
│   └── order_api.rs
│
├── .gitignore
├── Cargo.toml
├── docker-compose.yml
└── Dockerfile
```

# Feature List

---
This list is structured around the capabilities your backend service would need to provide via APIs.

## 👤 User & Auth Service

---
- [ ] Core: User registration, login (e.g., with JWT tokens), password reset, and profile management.
- [ ] Data: Manages user data, addresses, and authentication.

## 📦 Product Catalog Service

---
- [ ] Product Management (Admin): Full CRUD (Create, Read, Update, Delete) for products.
- [ ] Category Management (Admin): Organize products into categories and subcategories.
- [ ] Image Management (Admin): Ability to add multiple images per product.
- [ ] Public Access: Endpoints for clients to fetch products by category, search by name, and retrieve detailed product information (including images and average rating).

## 🛒 Persistent Shopping Cart Service

---
- [ ] Functionality: Add items, update quantities, and remove items from a cart. The cart's state is saved in the database, allowing it to persist between user sessions or devices.

## 📝 Order & Checkout Service

---
- [ ] Order Creation: Convert a user's shopping cart into a formal order.
- [ ] Order History: Allows users to retrieve a list of their past and current orders.
- [ ] Order Management (Admin): View all orders and update their status (e.g., processing, shipped, canceled).

## ⭐ Reviews & Wishlist Service

---
- [ ] Reviews: Allow authenticated users to submit reviews (rating and text) for products they've purchased. Fetch all reviews for a given product.
- [ ] Wishlist: Enable users to add or remove products from a personal wishlist.
