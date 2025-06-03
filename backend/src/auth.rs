use axum::{extract::State, Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::thread_rng;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use crate::models::user::{User};
use crate::AppState;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
    // All routes are relative, as required for Axum's .nest()
}

async fn signup(State(pool): State<PgPool>, Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, (axum::http::StatusCode, String)> {
    // Check if username already exists
    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if existing.is_some() {
        return Err((axum::http::StatusCode::CONFLICT, "Username already exists".to_string()));
    }

    let salt = SaltString::generate(&mut thread_rng());
    let password_hash = Argon2::default().hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .bind("candidate") // default role
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let token = create_jwt(&user)?;
    Ok(Json(AuthResponse { token, user }))
}

async fn login(State(pool): State<PgPool>, Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))?;
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Invalid password hash".to_string()))?;
    Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))?;
    let token = create_jwt(&user)?;
    Ok(Json(AuthResponse { token, user }))
}

fn create_jwt(user: &User) -> Result<String, (axum::http::StatusCode, String)> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let claims = Claims {
        sub: user.id,
        role: user.role.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
}
