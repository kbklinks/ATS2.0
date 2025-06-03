use axum::{extract::Path, extract::State, Json, Router, routing::{get, post, put, delete}};
use crate::{AppState, models::application::*, auth_middleware};
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_applications).post(create_application))
        .route("/:id", get(get_application).put(update_application).delete(delete_application))
        .route("/:id/stage", put(update_stage))
}

async fn list_applications(State(state): State<AppState>) -> Result<Json<Vec<Application>>, (axum::http::StatusCode, String)> {
    let applications = sqlx::query_as::<_, Application>("SELECT * FROM applications")
        .fetch_all(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(applications))
}

async fn get_application(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Application>, (axum::http::StatusCode, String)> {
    let application = sqlx::query_as::<_, Application>("SELECT * FROM applications WHERE id = $1")
        .bind(id)
        .fetch_one(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(application))
}

async fn create_application(State(state): State<AppState>, Json(payload): Json<CreateApplication>) -> Result<Json<Application>, (axum::http::StatusCode, String)> {
    let application = sqlx::query_as::<_, Application>(
        "INSERT INTO applications (job_id, candidate_id, referred_by, resume_url, cover_letter_url, notes) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(payload.job_id)
    .bind(payload.candidate_id)
    .bind(payload.referred_by)
    .bind(payload.resume_url)
    .bind(payload.cover_letter_url)
    .bind(payload.notes)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(application))
}

async fn update_application(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<CreateApplication>) -> Result<Json<Application>, (axum::http::StatusCode, String)> {
    let application = sqlx::query_as::<_, Application>(
        "UPDATE applications SET job_id = $1, candidate_id = $2, referred_by = $3, resume_url = $4, cover_letter_url = $5, notes = $6, updated_at = NOW() WHERE id = $7 RETURNING *"
    )
    .bind(payload.job_id)
    .bind(payload.candidate_id)
    .bind(payload.referred_by)
    .bind(payload.resume_url)
    .bind(payload.cover_letter_url)
    .bind(payload.notes)
    .bind(id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(application))
}

async fn delete_application(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    sqlx::query("DELETE FROM applications WHERE id = $1")
        .bind(id)
        .execute(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

#[derive(serde::Deserialize)]
struct UpdateStage {
    stage: ApplicationStage,
}

async fn update_stage(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateStage>) -> Result<Json<Application>, (axum::http::StatusCode, String)> {
    let application = sqlx::query_as::<_, Application>(
        "UPDATE applications SET stage = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(payload.stage)
    .bind(id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(application))
}
