use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{
    error::Error,
    models::user::{UserCredentials, UserForRegister},
    web::{services::auth::AuthController, ApiError},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterPayload {
    username: String,
    password: String,
    email: String,
    role: String,
}

pub fn routes(controller: AuthController) -> Router {
    Router::new()
        .route("/login", post(auth_login))
        .route("/logout", post(auth_logout))
        .route("/register", post(auth_register))
        .with_state(controller)
}

async fn auth_login(
    cookies: Cookies,
    State(controller): State<AuthController>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    let user = UserCredentials::new(payload.username.clone().to_lowercase(), payload.password.clone());
    let response_user = controller.login(user).await.map_err(ApiError::from)?;

    let cookie = Cookie::build("token", response_user.token.clone())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add(cookie);

    Ok(Json(json!({
        "username": response_user.username,
        "token": response_user.token,
    })))
}

async fn auth_register(
    State(controller): State<AuthController>,
    payload: Json<RegisterPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - api_register", "HANDLER");

    let username = payload.username.clone().to_lowercase();
    let user = UserForRegister::new(
        username.clone(),
        payload.password.clone(),
        payload.email.clone(),
        payload.role.clone(),
    );
    let user = controller.register(user).await.map_err(ApiError::from)?;

    Ok(Json(json!({
        "id": user.fk_user_id,
        "username": username,
    })))
}

async fn auth_logout(cookies: Cookies) -> Result<Json<Value>> {
    println!("->> {:<12} - api_logout", "HANDLER");

    let token_cookie = cookies
        .get("token")
        .and_then(|c| c.value().parse::<String>().ok());

    let remove_cookie = Cookie::build("token", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    if token_cookie.is_none() {
        println!("->> {:<12} - api_logout: no token cookie found", "HANDLER")
    } else {
        cookies.remove(remove_cookie);
    }

    Ok(Json(json!({
        "message": "Successfully logged out",
    })))
}
