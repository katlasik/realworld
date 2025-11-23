mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_register_creates_new_user() {
    // Given
    let app = common::create_test_app().await;
    let payload = json!({
        "user": {
            "username": "testuser",
            "email": "test@example.com",
            "password": "testpass123"
        }
    });

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["user"]["email"], "test@example.com");
    assert_eq!(body["user"]["username"], "testuser");
    assert!(body["user"]["token"].is_string());
}

#[tokio::test]
async fn test_register_with_invalid_email_fails() {
    // Given
    let app = common::create_test_app().await;
    let payload = json!({
        "user": {
            "username": "testuser",
            "email": "invalid-email",
            "password": "testpass123"
        }
    });

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_login_with_valid_credentials() {
    // Given
    let app = common::create_test_app().await;

    // First register a user
    let register_payload = json!({
        "user": {
            "username": "loginuser",
            "email": "login@example.com",
            "password": "loginpass123"
        }
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now login
    let login_payload = json!({
        "user": {
            "email": "login@example.com",
            "password": "loginpass123"
        }
    });

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["user"]["email"], "login@example.com");
    assert!(body["user"]["token"].is_string());
}

#[tokio::test]
async fn test_login_with_invalid_password_fails() {
    // Given
    let app = common::create_test_app().await;

    // First register a user
    let register_payload = json!({
        "user": {
            "username": "wrongpassuser",
            "email": "wrongpass@example.com",
            "password": "correctpass123"
        }
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&register_payload).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Try to login with wrong password
    let login_payload = json!({
        "user": {
            "email": "wrongpass@example.com",
            "password": "wrongpassword"
        }
    });

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_with_nonexistent_email_fails() {
    // Given
    let app = common::create_test_app().await;
    let login_payload = json!({
        "user": {
            "email": "nonexistent@example.com",
            "password": "somepassword"
        }
    });

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
