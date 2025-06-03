use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, extract::State};
use crate::auth::{decode_jwt, Claims};
use crate::AppState;
use std::sync::Arc;

use axum::body::Body;

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("authorization").and_then(|v| v.to_str().ok());
    let token = match auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let token_data = decode_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

pub async fn require_role(
    role: &'static str,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;
    if claims.role != role {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
