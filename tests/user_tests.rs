mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_current_user_with_valid_token() {
    // Given
    let app = common::create_test_app().await;

    // First register a user to get a token
    let register_payload = json!({
        "user": {
            "username": "currentuser",
            "email": "current@example.com",
            "password": "currentpass123"
        }
    });

    let register_response = app
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

    let body = axum::body::to_bytes(register_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/user")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
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

    assert_eq!(body["user"]["email"], "current@example.com");
    assert_eq!(body["user"]["username"], "currentuser");
}

#[tokio::test]
async fn test_get_current_user_without_token_fails() {
    // Given
    let app = common::create_test_app().await;

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_current_user_with_invalid_token_fails() {
    // Given
    let app = common::create_test_app().await;

    // When
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/user")
                .header("Authorization", "Bearer invalid.token.here")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_user() {

    // Given
    let app = common::create_test_app().await;

    // First register a user to get a token
    let register_payload = json!({
        "user": {
            "username": "currentuser",
            "email": "current@example.com",
            "password": "currentpass123"
        }
    });

    let register_response = app
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

    let body = axum::body::to_bytes(register_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = body["user"]["token"].as_str().unwrap();

    // When
    let update_payload = json!({
        "user": {
            "email": "newemail@example.com",
            "username": "newusername",
            "bio": "Updated bio",
            "image": "https://example.com/image.jpg"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/user")
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
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

    assert_eq!(body["user"]["email"], "newemail@example.com");
    assert_eq!(body["user"]["username"], "newusername");
}
