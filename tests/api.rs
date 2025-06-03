use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_auth_signup_and_login() {
    let client = Client::new();
    // Signup
    let res = client.post("http://localhost:8080/api/auth/signup")
        .json(&json!({"username": "testuser", "password": "testpass"}))
        .send().await.unwrap();
    assert!(res.status().is_success(), "Signup failed: {:?}", res.text().await);
    let body: serde_json::Value = res.json().await.unwrap();
    let token = body["token"].as_str().unwrap().to_string();
    // Login
    let res = client.post("http://localhost:8080/api/auth/login")
        .json(&json!({"username": "testuser", "password": "testpass"}))
        .send().await.unwrap();
    assert!(res.status().is_success(), "Login failed: {:?}", res.text().await);
    let body: serde_json::Value = res.json().await.unwrap();
    let login_token = body["token"].as_str().unwrap().to_string();
    assert_eq!(token, login_token);
}

#[tokio::test]
async fn test_protected_users_endpoint() {
    let client = Client::new();
    // Login to get JWT
    let res = client.post("http://localhost:8080/api/auth/login")
        .json(&json!({"username": "testuser", "password": "testpass"}))
        .send().await.unwrap();
    let body: serde_json::Value = res.json().await.unwrap();
    let token = body["token"].as_str().unwrap();
    // Access protected endpoint without JWT
    let res = client.get("http://localhost:8080/api/users").send().await.unwrap();
    assert_eq!(res.status(), 401);
    // Access with JWT
    let res = client.get("http://localhost:8080/api/users")
        .bearer_auth(token)
        .send().await.unwrap();
    assert!(res.status().is_success(), "Users endpoint failed: {:?}", res.text().await);
}
