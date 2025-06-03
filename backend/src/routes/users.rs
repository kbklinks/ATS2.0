use axum::{extract::Path, extract::State, Json, Router, routing::{get, post, put, delete}};
use crate::{AppState, models::user::*, auth_middleware};
use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(users))
}

async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(user))
}

async fn create_user(State(state): State<AppState>, Json(payload): Json<CreateUser>) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.username)
    .bind(&payload.password) // In production, hash the password!
    .bind(&payload.role)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(user))
}

async fn update_user(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<CreateUser>) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1, password_hash = $2, role = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&payload.username)
    .bind(&payload.password)
    .bind(&payload.role)
    .bind(id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(user))
}

async fn delete_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&*state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}
