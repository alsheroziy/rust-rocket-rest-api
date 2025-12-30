use sqlx::SqlitePool;
use crate::domain::models::{Product, CreateProductRequest};
use anyhow::Result;
use sqlx::Row;

pub struct ProductRepository {
    pool: SqlitePool,
}

impl ProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_products(&self) -> Result<Vec<Product>> {
        let products = sqlx::query_as::<_, Product>(
            "SELECT id, name, price, description FROM products"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(products)
    }

    pub async fn create_product(&self, req: CreateProductRequest) -> Result<Product> {
        let id = sqlx::query(
            "INSERT INTO products (name, price, description) VALUES (?, ?, ?) RETURNING id"
        )
        .bind(&req.name)
        .bind(req.price)
        .bind(&req.description)
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("id");

        Ok(Product {
            id,
            name: req.name,
            price: req.price,
            description: req.description,
        })
    }

    pub async fn get_product(&self, id: i64) -> Result<Option<Product>> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT id, name, price, description FROM products WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(product)
    }

    pub async fn delete_product(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
