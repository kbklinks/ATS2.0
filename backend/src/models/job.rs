use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub posted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJob {
    pub title: String,
    pub description: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
}
