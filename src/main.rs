#[macro_use] extern crate rocket;

mod api;
mod domain;
mod repository;
mod infrastructure;
mod error;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::repository::product_repo::ProductRepository;
use crate::repository::user_repo::UserRepository;
use crate::api::{product_handler, auth_handler};
use crate::domain::models::{Product, CreateProductRequest, RegisterRequest, LoginRequest, AuthResponse};
use sqlx::sqlite::SqlitePoolOptions;
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, Http};

#[derive(OpenApi)]
#[openapi(
    paths(
        product_handler::list_products,
        product_handler::get_product,
        product_handler::create_product,
        product_handler::delete_product,
        auth_handler::register,
        auth_handler::login
    ),
    components(
        schemas(Product, CreateProductRequest, RegisterRequest, LoginRequest, AuthResponse)
    ),
    tags(
        (name = "Shop", description = "Shop management endpoints"),
        (name = "Auth", description = "Authentication endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}

#[launch]
async fn rocket() -> _ {
    // Load env
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initialize DB Pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run Migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let product_repo = ProductRepository::new(pool.clone());
    let user_repo = UserRepository::new(pool.clone());

    rocket::build()
        .manage(product_repo)
        .manage(user_repo)
        .mount("/", routes![
            product_handler::list_products, 
            product_handler::get_product, 
            product_handler::create_product,
            product_handler::delete_product,
            auth_handler::register,
            auth_handler::login
        ])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
