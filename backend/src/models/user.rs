use crate::models::Result;

pub struct User {
    pub fk_user_id: i32,
    pub email: String,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub tags: Option<Vec<String>>,
    pub resume: Option<String>,
    pub role: String,
}

pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

impl UserCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct UserForRegister {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

impl UserForRegister {
    pub fn new(username: String, password: String, email: String, role: String) -> Self {
        Self {
            username,
            password,
            email,
            role,
        }
    }
}

pub struct UserForUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub tags: Option<Vec<String>>,
    pub resume: Option<String>,
    pub role: Option<String>,
}

pub struct UserModel {
    pub db_pool: sqlx::PgPool,
}

impl UserModel {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { db_pool: pool }
    }
}

impl UserModel {
    pub async fn create(&self, user: UserForRegister) -> Result<User> {
        let mut transaction = self.db_pool.begin().await?;

        let user_id = sqlx::query!(
            r#"
            INSERT INTO 
                users_credentials (username, password_hash) 
            VALUES 
                ($1, $2) 
            RETURNING user_id
            "#,
            user.username,
            user.password
        )
        .fetch_one(&mut *transaction)
        .await?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO 
                users (fk_user_id, email, role) 
            VALUES 
                ($1, $2, $3) 
            RETURNING *
            "#,
            user_id.user_id,
            user.email,
            user.role
        )
        .fetch_one(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(user)
    }

    pub async fn update(&self, id: i32, user: UserForUpdate) -> Result<User> {
        todo!();
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        let mut transaction = self.db_pool.begin().await?;

        sqlx::query!(
            r#"
            DELETE FROM 
                users 
            WHERE 
                fk_user_id = $1
            "#,
            id
        )
        .execute(&mut *transaction)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM 
                users_credentials 
            WHERE 
                user_id = $1
            "#,
            id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn check_if_username_exists(&self, username: &String) -> Result<bool> {
        let user = sqlx::query!(
            r#"
            SELECT username
            FROM users_credentials
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.db_pool)
        .await?;

        match user {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn find_by_username(&self, username: &String) -> Result<User> {
        // Join users and users_credentials tables
        let user = sqlx::query!(
            r#"
            SELECT users.fk_user_id, users.email, users.location, users.bio, users.tags, users.resume, users.role
            FROM users
            INNER JOIN users_credentials
            ON users.fk_user_id = users_credentials.user_id
            WHERE users_credentials.username = $1
            "#,
            username
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(User {
            fk_user_id: user.fk_user_id,
            email: user.email,
            location: user.location,
            bio: user.bio,
            tags: user.tags,
            resume: user.resume,
            role: user.role,
        })
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User> {
        todo!();
    }

    pub async fn find_hashed_password_by_username(&self, username: String) -> Result<String> {
        let hashed_password = sqlx::query!(
            r#"
            SELECT password_hash
            FROM users_credentials
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.db_pool)
        .await?
        .password_hash;

        Ok(hashed_password)
    }
}
