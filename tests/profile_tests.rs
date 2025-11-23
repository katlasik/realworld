mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

async fn register_user(app: axum::Router, username: &str, email: &str, password: &str) -> String {
    let payload = json!({
        "user": {
            "username": username,
            "email": email,
            "password": password
        }
    });

    let response = app
        .clone()
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    body["user"]["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_get_profile_without_authentication() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "testuser", "test@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/profiles/testuser")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["profile"]["username"], "testuser");
    assert_eq!(body["profile"]["following"], false);
}

#[tokio::test]
async fn test_get_profile_with_authentication() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "profileuser", "profile@example.com", "password123").await;
    let token = register_user(app.clone(), "viewer", "viewer@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/profiles/profileuser")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["profile"]["username"], "profileuser");
    assert_eq!(body["profile"]["following"], false);
}

#[tokio::test]
async fn test_get_nonexistent_profile_returns_404() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/profiles/nonexistentuser")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_follow_user() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "usertofollow", "tofollow@example.com", "password123").await;
    let follower_token = register_user(app.clone(), "follower", "follower@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/usertofollow/follow")
                .header("authorization", format!("Bearer {}", follower_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["profile"]["username"], "usertofollow");
    assert_eq!(body["profile"]["following"], true);
}

#[tokio::test]
async fn test_follow_user_without_authentication_fails() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "usertofollow", "tofollow@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/usertofollow/follow")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_follow_nonexistent_user_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "follower", "follower@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/nonexistentuser/follow")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_unfollow_user() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "usertounfollow", "tounfollow@example.com", "password123").await;
    let follower_token = register_user(app.clone(), "follower", "follower@example.com", "password123").await;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/usertounfollow/follow")
                .header("authorization", format!("Bearer {}", follower_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/profiles/usertounfollow/follow")
                .header("authorization", format!("Bearer {}", follower_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["profile"]["username"], "usertounfollow");
    assert_eq!(body["profile"]["following"], false);
}

#[tokio::test]
async fn test_unfollow_user_without_authentication_fails() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "usertounfollow", "tounfollow@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/profiles/usertounfollow/follow")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_unfollow_nonexistent_user_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "follower", "follower@example.com", "password123").await;

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/profiles/nonexistentuser/follow")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_profile_shows_following_status() {
    let app = common::create_test_app().await;
    register_user(app.clone(), "targetuser", "target@example.com", "password123").await;
    let viewer_token = register_user(app.clone(), "viewer", "viewer@example.com", "password123").await;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/targetuser/follow")
                .header("authorization", format!("Bearer {}", viewer_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/profiles/targetuser")
                .header("authorization", format!("Bearer {}", viewer_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["profile"]["username"], "targetuser");
    assert_eq!(body["profile"]["following"], true);
}
