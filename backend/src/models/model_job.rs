use crate::web::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Job {
    id: i32,
    title: String,
    description: Option<String>,
    fk_company_id: i32,
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
pub struct JobController {
    pub pool: sqlx::PgPool,
}

impl JobController {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl JobController {
    pub async fn create(&self, job: JobForCreate) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                INSERT INTO 
                    job (title, description) 
                VALUES 
                    ($1, $2) 
                RETURNING *
            "#,
            &job.title,
            job.description.as_deref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
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

    pub async fn update(&self, job_id: i32, job: JobForUpdate) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                UPDATE 
                    job 
                SET
                    title = COALESCE($1, title),
                    description = COALESCE($2, description)
                WHERE 
                    id = $3
                RETURNING *
            "#,
            job.title.as_deref(),
            job.description.as_deref(),
            &job_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    pub async fn delete(&self, job_id: i32) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
                DELETE FROM 
                    job 
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            &job_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }
}
