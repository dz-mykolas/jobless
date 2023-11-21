use axum::{
    extract::{Extension, Path},
    http::{Request, Response, StatusCode},
    response::Json,
    Router,
};

use crate::db;


struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub is_admin: bool,
}

struct Claims {
    pub sub: String,
    pub exp: usize,
    pub is_admin: bool,
}

async fn register(Extension(pool): Extension<sqlx::PgPool>, Json(payload): Json<User>) -> Result<Json<String>, StatusCode> {
    let (
        email,
        password,
        first_name,
        is_admin,
    ) = (
        payload.email,
        payload.password,
        payload.first_name,
        payload.is_admin,
    );

    

    Ok(Json("".to_string()))
}

async fn login(Json(payload): Json<LoginPayload>) -> Result<Json<User>, StatusCode> {
    println!("->> {:<12} - api_login", "HANDLER");

    let user = User {
        id: 1,
        email: payload.email,
        password: payload.password,
        first_name: "Placeholder".to_string(),
        is_admin: false,
    };

    Ok(Json(user))
}