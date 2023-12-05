use axum::response::IntoResponse;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{error::MainErrorResponse, models::user::User, web::routes::Role};

#[derive(Debug, Serialize)]
pub enum TokenError {
    InvalidToken,
    ExpiredToken,
    InvalidSignature,
    #[serde(skip)]
    EncodeError(jsonwebtoken::errors::Error),
}

impl From<jsonwebtoken::errors::Error> for TokenError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => Self::InvalidToken,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::ExpiredToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => Self::InvalidSignature,
            _ => Self::EncodeError(err),
        }
    }
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        println!("TokenError: {:?}", self);
        match self {
            TokenError::InvalidToken | TokenError::ExpiredToken | TokenError::InvalidSignature => {
                MainErrorResponse::new(
                    "INVALID_TOKEN",
                    "The provided token is invalid. Please login again.",
                )
                .into_response(axum::http::StatusCode::UNAUTHORIZED)
            }
            TokenError::EncodeError(_) => MainErrorResponse::new(
                "INTERNAL_SERVER_ERROR",
                "Something went wrong. Please try again later.",
            )
            .into_response(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub type Result<T> = core::result::Result<T, TokenError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // Subject
    pub exp: usize,      // Expiration time
    pub iat: usize,      // Issued at
    pub iss: String,     // Issuer
    pub user_role: Role, // User role
}

pub fn create_token(user: &User) -> Result<String> {
    let current_time = Utc::now();
    let iat = current_time.timestamp() as usize;
    // 60s leeway added by default: https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html#structfield.leeway
    let exp = (current_time + chrono::Duration::seconds(10)).timestamp() as usize;

    let claims = Claims {
        sub: user.fk_user_id.to_string(),
        exp,
        iat,
        iss: "jobless_app".to_string(),
        user_role: Role::from(user.role.clone()),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    if secret.contains("placeholder_") {
        panic!("JWT_SECRET in .env file is still a placeholder. Please update it with your actual JWT secret.");
    }
    let secret = EncodingKey::from_secret(secret.as_bytes());

    let token = encode(&Header::default(), &claims, &secret)?;

    Ok(token)
}

pub fn get_claims(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let claims =
        jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))?
            .claims;

    Ok(claims)
}
