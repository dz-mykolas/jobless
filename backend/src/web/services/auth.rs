use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::models::{
    user::{User, UserCredentials, UserForRegister, UserModel},
    ModelError,
};

use crate::web::services::jwt::{self, TokenError};

use super::jwt::Claims;

pub type Result<T> = core::result::Result<T, AuthError>;

#[derive(Debug, Serialize)]
pub struct UserForResponse {
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub enum AuthError {
    Forbidden,
    Unauthorized,
    UserAlreadyExists,
    BadUsername,
    BadPassword,
    UserNotFound,
    IncorrectPassword,
    ModelError(ModelError),
    TokenError(TokenError),
    #[serde(skip)]
    BcryptError(bcrypt::BcryptError),
}

impl From<ModelError> for AuthError {
    fn from(err: ModelError) -> Self {
        match err {
            ModelError::NotFound => Self::UserNotFound,
            _ => Self::ModelError(err),
        }
    }
}

impl From<TokenError> for AuthError {
    fn from(err: TokenError) -> Self {
        Self::TokenError(err)
    }
}

impl From<bcrypt::BcryptError> for AuthError {
    fn from(err: bcrypt::BcryptError) -> Self {
        Self::BcryptError(err)
    }
}

#[derive(Clone)]
pub struct AuthController {
    pub db_pool: Pool<Postgres>,
}

impl AuthController {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self { db_pool }
    }

    pub async fn login(&self, user: UserCredentials) -> Result<UserForResponse> {
        if !validate_username(&user.username) {
            return Err(AuthError::UserNotFound);
        }

        if !validate_password(&user.password) {
            return Err(AuthError::IncorrectPassword);
        }

        let user_model = UserModel::new(self.db_pool.clone());

        let username = user.username.clone();
        if !verify_user(&user_model, user).await? {
            println!("Incorrect password");
            return Err(AuthError::IncorrectPassword);
        }

        let user = user_model.find_by_username(&username).await?;
        let token = jwt::create_token(&user)?;
        let user = UserForResponse {
            username: username,
            token,
        };

        Ok(user)
    }

    pub async fn register(&self, user: UserForRegister) -> Result<User> {
        if !validate_username(&user.username) {
            return Err(AuthError::BadUsername);
        }

        if !validate_password(&user.password) {
            return Err(AuthError::BadPassword);
        }

        let user_model = UserModel::new(self.db_pool.clone());

        let user = match user_model.check_if_username_exists(&user.username).await? {
            true => return Err(AuthError::UserAlreadyExists),
            false => {
                let password_hash = hash_password(&user.password)?;
                println!("password_hash: {}", password_hash);
                let user = user_model
                    .create(UserForRegister {
                        password: password_hash,
                        ..user
                    })
                    .await?;
                user
            }
        };

        Ok(user)
    }

    pub async fn verify(&self, token: &str) -> Result<Claims> {
        let claims = jwt::validate_token(token)?;

        Ok(claims)
    }
}

fn validate_username(username: &str) -> bool {
    // Check for ascii only
    if username.contains(|c: char| !c.is_ascii()) {
        return false;
    }
    username.len() > 3 && username.len() < 20
}

fn validate_password(password: &str) -> bool {
    if !password.contains(|c: char| c.is_ascii_uppercase()) {
        return false;
    }
    if !password.contains(|c: char| c.is_ascii_lowercase()) {
        return false;
    }
    if !password.contains(|c: char| c.is_numeric()) {
        return false;
    }
    password.len() > 8 && password.len() < 20
}

fn hash_password(password: &str) -> Result<String> {
    Ok(bcrypt::hash(password, 8)?)
}

async fn verify_user(user_model: &UserModel, user: UserCredentials) -> Result<bool> {
    let password_hash = user_model
        .find_hashed_password_by_username(user.username)
        .await?;

    Ok(bcrypt::verify(user.password, &password_hash)?)
}
