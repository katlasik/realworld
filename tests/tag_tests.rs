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
async fn test_get_tags_returns_empty_list_initially() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/tags")
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

    assert_eq!(body["tags"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_tags_returns_tags_from_articles() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let article1 = json!({
        "article": {
            "title": "Rust Programming",
            "description": "Learn Rust",
            "body": "Content",
            "tagList": ["rust", "programming"]
        }
    });

    let article2 = json!({
        "article": {
            "title": "Web Development",
            "description": "Learn Web Dev",
            "body": "Content",
            "tagList": ["web", "programming", "axum"]
        }
    });

    for payload in [article1, article2] {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/tags")
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

    let tags = body["tags"].as_array().unwrap();
    assert!(tags.len() >= 4);

    let tag_strings: Vec<String> = tags
        .iter()
        .map(|t| t.as_str().unwrap().to_string())
        .collect();

    assert!(tag_strings.contains(&"rust".to_string()));
    assert!(tag_strings.contains(&"programming".to_string()));
    assert!(tag_strings.contains(&"web".to_string()));
    assert!(tag_strings.contains(&"axum".to_string()));
}

#[tokio::test]
async fn test_get_tags_does_not_require_authentication() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let article = json!({
        "article": {
            "title": "Public Tags Article",
            "description": "Tags should be public",
            "body": "Content",
            "tagList": ["public", "tags"]
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&article).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/tags")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_tags_returns_unique_tags() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let article1 = json!({
        "article": {
            "title": "Article One",
            "description": "First article",
            "body": "Content",
            "tagList": ["duplicate", "unique1"]
        }
    });

    let article2 = json!({
        "article": {
            "title": "Article Two",
            "description": "Second article",
            "body": "Content",
            "tagList": ["duplicate", "unique2"]
        }
    });

    for payload in [article1, article2] {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/tags")
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

    let tags = body["tags"].as_array().unwrap();
    let tag_strings: Vec<String> = tags
        .iter()
        .map(|t| t.as_str().unwrap().to_string())
        .collect();

    let duplicate_count = tag_strings.iter().filter(|t| *t == "duplicate").count();
    assert_eq!(duplicate_count, 1);
}
