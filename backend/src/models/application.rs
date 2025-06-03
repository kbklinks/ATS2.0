use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::Type;

pub type ApplicationStage = String;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    pub id: Uuid,
    pub job_id: i32,
    pub candidate_id: Uuid,
    pub stage: ApplicationStage,
    pub referred_by: Option<Uuid>,
    pub application_date: DateTime<Utc>,
    pub resume_url: Option<String>,
    pub cover_letter_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApplication {
    pub job_id: i32,
    pub candidate_id: Uuid,
    pub referred_by: Option<Uuid>,
    pub resume_url: Option<String>,
    pub cover_letter_url: Option<String>,
    pub notes: Option<String>,
}
