use rocket::serde::json::Json;
use rocket::{get, post, delete, State};
use crate::domain::models::{Product, CreateProductRequest};
use crate::repository::product_repo::ProductRepository;
use crate::infrastructure::auth_guard::AuthenticatedUser;

#[utoipa::path(
    get,
    path = "/products",
    tag = "Shop",
    responses(
        (status = 200, description = "List all products", body = [Product])
    )
)]
#[get("/products")]
pub async fn list_products(repo: &State<ProductRepository>) -> Json<Vec<Product>> {
    // Handling error by returning empty list for simplicity in this demo, real app needs error handling
    let products = repo.list_products().await.unwrap_or_default();
    Json(products)
}

#[utoipa::path(
    get,
    path = "/products/{id}",
    tag = "Shop",
    params(
        ("id" = i64, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product found", body = Product),
        (status = 404, description = "Product not found")
    )
)]
#[get("/products/<id>")]
pub async fn get_product(id: i64, repo: &State<ProductRepository>) -> Option<Json<Product>> {
    repo.get_product(id).await.ok().flatten().map(Json)
}

#[utoipa::path(
    post,
    path = "/products",
    tag = "Shop",
    request_body = CreateProductRequest,
    responses(
        (status = 200, description = "Create a new product", body = Product),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
#[post("/products", format = "json", data = "<req>")]
pub async fn create_product(
    req: Json<CreateProductRequest>, 
    repo: &State<ProductRepository>,
    _user: AuthenticatedUser
) -> Result<Json<Product>, rocket::http::Status> {
    match repo.create_product(req.into_inner()).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}

#[utoipa::path(
    delete,
    path = "/products/{id}",
    tag = "Shop",
    params(
        ("id" = i64, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product deleted"),
        (status = 404, description = "Product not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt_auth" = [])
    )
)]
#[delete("/products/<id>")]
pub async fn delete_product(
    id: i64, 
    repo: &State<ProductRepository>,
    _user: AuthenticatedUser
) -> Result<rocket::http::Status, rocket::http::Status> {
    match repo.delete_product(id).await {
        Ok(true) => Ok(rocket::http::Status::Ok),
        Ok(false) => Err(rocket::http::Status::NotFound),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}
