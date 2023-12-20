use crate::models::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub fk_company_id: i32,
    pub fk_user_id: i32,
}

#[derive(Deserialize)]
pub struct JobForCreate {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct JobForUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct JobModel {
    pub pool: sqlx::PgPool,
}

impl JobModel {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl JobModel {
    pub async fn create(&self, job: JobForCreate, company_id: i32, user_id: i32) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                INSERT INTO job (title, description, fk_company_id, fk_user_id)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            job.title,
            job.description,
            company_id,
            &user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn get_by_company(&self, company_id: i32) -> Result<Vec<Job>> {
        let jobs = sqlx::query_as!(
            Job,
            r#"
                SELECT 
                    * 
                FROM 
                    job 
                WHERE 
                    fk_company_id = $1
            "#,
            &company_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    pub async fn get_all(&self) -> Result<Vec<Job>> {
        let jobs = sqlx::query_as!(
            Job,
            r#"
                SELECT 
                    * 
                FROM 
                    job
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    pub async fn get(&self, job_id: i32) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                SELECT 
                    * 
                FROM 
                    job 
                WHERE 
                    id = $1
            "#,
            &job_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn update(&self, job_id: i32, job: JobForUpdate, user_id: i32) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                UPDATE 
                    job 
                SET 
                    title = COALESCE($2, title), 
                    description = COALESCE($3, description) 
                WHERE 
                    id = $1
                AND
                    ($4 = -1 OR fk_user_id = $4)
                RETURNING *
            "#,
            &job_id,
            job.title,
            job.description,
            &user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn delete(&self, job_id: i32, user_id: i32) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                DELETE FROM 
                    job 
                WHERE
                    id = $1
                AND
                    ($2 = -1 OR fk_user_id = $2)
                RETURNING *
            "#,
            &job_id,
            &user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }
}
