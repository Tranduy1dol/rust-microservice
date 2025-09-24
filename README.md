# Shopping Cart Microservice

## Description

---

This repository implements simple RESTful API for an e-commerce application contain service for seller. Another feature for billing and buyer will be implemented in the future

## Assignment requirements

---

| **Basic functionality**                                                      |                                               |
|------------------------------------------------------------------------------|-----------------------------------------------|
| Incorporating descriptive comments to enhance code readability.              | <span style="color:green">Done</span>         |
| Implementing tracing mechanisms for effective debugging.                     | <span style="color:green">Done</span>         |
| Writing comprehensive test cases to validate functionality.                  | <span style="color:yellow">Almost Done</span> |
| Utilizing version control with Git for code management.                      | <span style="color:green">Done</span>         |
| Structuring code in a logical and maintainable manner.                       | <span style="color:green">Done</span>         |
| Containerizing the application using Docker for portability and scalability. | <span style="color:green">Done</span>         |
| **Advance functionality**                                                    |                                               |
| Load Configuration from a File                                               | <span style="color:yellow">Almost Done</span> |
| Multiple Implementations                                                     | <span style="color:red">Not yet</span>        |
| Advanced Tracing                                                             | <span style="color:red">Not yet</span>        |
| CI/CD                                                                        | <span style="color:red">Not yet</span>        |
| Docker Image Optimization                                                    | <span style="color:red">Not yet</span>        |

### Improvement

---

Because this is just a simple repository, that mean there are a lot feature can be set up in the future. Some suggestion feature:

- Implement feature for buyer and bill.
- Implement middleware for admin so that admin can delete bad accounts.
- Optimization Docker image.
- Add CI/CD pipeline for auto build, deploy and test.
- Implement front-end.


# Feature List

---
This list is structured around the capabilities your backend service would need to provide via APIs.

## 👤 User & Auth Service

---
- [x] Core: User registration, login (e.g., with JWT tokens), password reset, and profile management.
- [x] Data: Manages user data, addresses, and authentication.

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

# TODO List

---
- [ ] Implement feature lists.
- [ ] Implement e2e test & benchmark test.
- [ ] Add a CI/CD pipeline for auto build, deploy and test.
