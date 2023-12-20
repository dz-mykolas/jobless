use crate::models::Result;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "application_status", rename_all = "snake_case")]
pub enum ApplicationStatus {
    NotDeniedNotAccepted,
    Denied,
    Accepted,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct Application {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: ApplicationStatus,
    pub fk_job_id: i32,
    pub fk_user_id: i32,
}

#[derive(Deserialize)]
pub struct ApplicationForCreate {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ApplicationForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct ApplicationModel {
    pub pool: sqlx::PgPool,
}

impl ApplicationModel {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl ApplicationModel {
    pub async fn create(
        &self,
        application: ApplicationForCreate,
        job_id: i32,
        user_id: i32,
    ) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                INSERT INTO 
                    application (name, description, fk_job_id, fk_user_id) 
                VALUES 
                    ($1, $2, $3, $4) 
                RETURNING 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
            "#,
            &application.name,
            &application.description,
            &job_id,
            &user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(application)
    }

    pub async fn get_by_job(&self, job_id: i32, user_id: i32) -> Result<Vec<Application>> {
        let applications = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    a.id,
                    a.name,
                    a.description,
                    a.fk_job_id,
                    a.fk_user_id,
                    a.status AS "status: ApplicationStatus"
                FROM 
                    application AS a
                JOIN 
                    job AS j ON a.fk_job_id = j.id
                WHERE 
                    a.fk_job_id = $1
                AND 
                    ($2 = -1 OR a.fk_user_id = $2 OR j.fk_user_id = $2)
            "#,
            job_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(applications)
    }
    
    pub async fn get_all(&self) -> Result<Vec<Application>> {
        let applications = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
                FROM 
                    application
            "#
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(applications)
    }

    pub async fn get(&self, id: i32, user_id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
                FROM 
                    application 
                WHERE 
                    id = $1 AND ($2 = -1 OR fk_user_id = $2)
            "#,
            id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(application)
    }
    
    pub async fn get_by_user(&self, user_id: i32) -> Result<Vec<Application>> {
        let applications = sqlx::query_as!(
            Application,
            r#"
                SELECT 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
                FROM 
                    application 
                WHERE 
                    ($1 = -1 OR fk_user_id = $1)
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(applications)
    }
    

    pub async fn update(
        &self,
        id: i32,
        application: ApplicationForUpdate,
        user_id: i32,
    ) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                UPDATE 
                    application 
                SET 
                    name = COALESCE($1, name), 
                    description = COALESCE($2, description) 
                WHERE 
                    id = $3 AND ($4 = -1 OR fk_user_id = $4) 
                RETURNING 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
            "#,
            application.name,
            application.description,
            id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(application)
    }
    
    pub async fn delete(&self, id: i32, user_id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                DELETE FROM 
                    application 
                WHERE 
                    id = $1 AND ($2 = -1 OR fk_user_id = $2) 
                RETURNING 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
            "#,
            id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(application)
    }
    

    pub async fn reject(&self, application_id: i32, user_id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
                UPDATE
                    application
                SET 
                    status = 'denied'::application_status
                WHERE 
                    id = $1 
                AND 
                    ($2 = -1 OR fk_user_id = (SELECT fk_user_id FROM job WHERE id = fk_job_id AND fk_user_id = $2))
                RETURNING 
                    id,
                    name,
                    description,
                    fk_job_id,
                    fk_user_id,
                    status AS "status: ApplicationStatus"
            "#,
            application_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(application)
    }

    pub async fn accept(&self, application_id: i32, user_id: i32) -> Result<Application> {
        let application = sqlx::query_as!(
            Application,
            r#"
            UPDATE
                application
            SET 
                status = 'accepted'::application_status
            WHERE 
                id = $1
            AND 
                ($2 = -1 OR EXISTS (
                    SELECT 1 
                    FROM job 
                    WHERE id = application.fk_job_id AND fk_user_id = $2
                ))
            RETURNING 
                id,
                name,
                description,
                fk_job_id,
                fk_user_id,
                status AS "status: ApplicationStatus"
            "#,
            application_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(application)
    }    
}
