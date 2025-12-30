use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct Product {
    #[schema(example = 1)]
    pub id: i64, // SQLite uses i64/INTEGER
    #[schema(example = "Apple")]
    pub name: String,
    #[schema(example = 1.99)]
    pub price: f64,
    #[schema(example = "Fresh Red Apple")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct User {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub fullname: String,
    #[serde(skip)]
    pub password_hash: String,
    #[schema(example = "user")]
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RegisterRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "John Doe")]
    pub fullname: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginRequest {
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub email: String,
    pub fullname: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // email
    pub exp: usize,
}
