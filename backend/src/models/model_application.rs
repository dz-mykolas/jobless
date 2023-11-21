use crate::web::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Application {
    id: i32,
    name: String,
    description: String,
    fk_job_id: i32,
}

#[derive(Deserialize)]
pub struct ApplicationForCreate {
    pub name: String,
    pub description: String,
    pub fk_job_id: i32,
}

#[derive(Deserialize)]
pub struct ApplicationForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct ApplicationController {
    pub pool: sqlx::PgPool,
}

impl ApplicationController {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl ApplicationController {
    pub async fn create(&self, application: ApplicationForCreate) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                INSERT INTO 
                    application (name, description, fk_job_id) 
                VALUES 
                    ($1, $2, $3) 
                RETURNING *
            "#,
            &application.name,
            &application.description,
            &application.fk_job_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(application)
    }

    pub async fn get_all(&self) -> Result<Vec<Application>> {
        let applications = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    * 
                FROM 
                    application
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(applications)
    }

    pub async fn get(&self, id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    * 
                FROM 
                    application 
                WHERE 
                    id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(application)
    }

    pub async fn update(&self, id: i32, application: ApplicationForUpdate) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                UPDATE 
                    application 
                SET 
                    name = COALESCE($1, name), 
                    description = COALESCE($2, description) 
                WHERE 
                    id = $3 
                RETURNING *
            "#,
            application.name,
            application.description,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(application)
    }

    pub async fn delete(&self, id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                DELETE FROM 
                    application 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(application)
    }
}
