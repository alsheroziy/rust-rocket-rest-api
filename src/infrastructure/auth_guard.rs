use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::domain::models::Claims;

#[derive(Debug)]
pub struct AuthenticatedUser {
    #[allow(dead_code)]
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].replace("Bearer ", "");
        // Ideally load secret from Config
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key_change_me".to_string());
        
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(token_data) => Outcome::Success(AuthenticatedUser { email: token_data.claims.sub }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
