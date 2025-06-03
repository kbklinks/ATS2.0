use axum::{routing::get, Router, Json};
use crate::AppState;
use serde_json::json;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(health))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}
