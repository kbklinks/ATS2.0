
mod db;
mod models;
mod routes;
mod auth;
mod auth_middleware;

use axum::{Router};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let pool = db::get_pool().await;
    let state = AppState { pool: Arc::new(pool) };

    let app = Router::new()
        .nest("/api/auth", auth::routes().with_state(state.clone()))
        .nest("/api/users", routes::users::routes().layer(axum::middleware::from_fn_with_state(auth_middleware::require_auth, state.clone())).with_state(state.clone()))
        .nest("/api/jobs", routes::jobs::routes().layer(axum::middleware::from_fn_with_state(auth_middleware::require_auth, state.clone())).with_state(state.clone()))
        .nest("/api/applications", routes::applications::routes().layer(axum::middleware::from_fn_with_state(auth_middleware::require_auth, state.clone())).with_state(state.clone()))
        .nest("/health", routes::health::routes().with_state(state.clone()))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Backend running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
