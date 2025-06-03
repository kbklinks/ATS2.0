use axum::{extract::Path, extract::State, Json, Router, routing::{get, post, put, delete}};
use crate::{AppState, models::job::*, auth_middleware};
use sqlx::PgPool;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_jobs).post(create_job))
        .route("/:id", get(get_job).put(update_job).delete(delete_job))
}

async fn list_jobs(State(state): State<AppState>) -> Result<Json<Vec<Job>>, (axum::http::StatusCode, String)> {
    let jobs = sqlx::query_as::<_, Job>("SELECT * FROM jobs")
        .fetch_all(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(jobs))
}

async fn get_job(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Job>, (axum::http::StatusCode, String)> {
    let job = sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
        .bind(id)
        .fetch_one(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(job))
}

async fn create_job(State(state): State<AppState>, Json(payload): Json<CreateJob>) -> Result<Json<Job>, (axum::http::StatusCode, String)> {
    let job = sqlx::query_as::<_, Job>(
        "INSERT INTO jobs (title, description, company, location) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.company)
    .bind(&payload.location)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(job))
}

async fn update_job(State(state): State<AppState>, Path(id): Path<i32>, Json(payload): Json<CreateJob>) -> Result<Json<Job>, (axum::http::StatusCode, String)> {
    let job = sqlx::query_as::<_, Job>(
        "UPDATE jobs SET title = $1, description = $2, company = $3, location = $4 WHERE id = $5 RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.company)
    .bind(&payload.location)
    .bind(id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(job))
}

async fn delete_job(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    sqlx::query("DELETE FROM jobs WHERE id = $1")
        .bind(id)
        .execute(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}
