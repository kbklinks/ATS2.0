use axum::{Router, routing::get};
use tower::ServiceExt; // for `oneshot` method

#[tokio::test]
async fn test_health() {
    let app = Router::new().route("/health", get(|| async { "OK" }));
    let response = app.oneshot(axum::http::Request::builder().uri("/health").body(axum::body::Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), 200);
}
