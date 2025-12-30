use rocket::serde::json::Json;
use rocket::{post, State};
use crate::domain::models::{RegisterRequest, LoginRequest, AuthResponse, Claims, User};
use crate::repository::user_repo::UserRepository;
use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Argon2
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Email already exists")
    )
)]
#[post("/auth/register", format = "json", data = "<req>")]
pub async fn register(req: Json<RegisterRequest>, repo: &State<UserRepository>) -> Result<Json<AuthResponse>, rocket::http::Status> {
    match repo.create_user(req.into_inner()).await {
        Ok(user) => generate_token(user),
        Err(_) => Err(rocket::http::Status::BadRequest),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/auth/login", format = "json", data = "<req>")]
pub async fn login(req: Json<LoginRequest>, repo: &State<UserRepository>) -> Result<Json<AuthResponse>, rocket::http::Status> {
    let user_opt = repo.find_by_email(&req.email).await.map_err(|_| rocket::http::Status::InternalServerError)?;
    
    if let Some(user) = user_opt {
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| rocket::http::Status::InternalServerError)?;
        if Argon2::default().verify_password(req.password.as_bytes(), &parsed_hash).is_ok() {
            return generate_token(user);
        }
    }
    
    Err(rocket::http::Status::Unauthorized)
}

fn generate_token(user: User) -> Result<Json<AuthResponse>, rocket::http::Status> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 3600; // 1 hour

    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    Ok(Json(AuthResponse {
        token,
        email: user.email,
        fullname: user.fullname,
    }))
}
