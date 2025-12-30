use sqlx::SqlitePool;
use crate::domain::models::{User, RegisterRequest};
use anyhow::Result;
use sqlx::Row;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, req: RegisterRequest) -> Result<User> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .to_string();

        let id = sqlx::query(
            "INSERT INTO users (email, fullname, password_hash) VALUES (?, ?, ?) RETURNING id"
        )
        .bind(&req.email)
        .bind(&req.fullname)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("id");

        Ok(User {
            id,
            email: req.email,
            fullname: req.fullname,
            password_hash,
            role: "user".to_string(),
        })
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, fullname, password_hash, role FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}
