use thiserror::Error;
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::serde_json;
use std::io::Cursor;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Product not found")]
    NotFound,
    #[error("Internal server error")]
    InternalError,
}

// Implement Responder for AppError to return proper HTTP responses
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            AppError::NotFound => Status::NotFound,
            AppError::InternalError => Status::InternalServerError,
        };

        let err_response = serde_json::json!({
            "error": self.to_string()
        });

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(err_response.to_string().len(), Cursor::new(err_response.to_string()))
            .ok()
    }
}
