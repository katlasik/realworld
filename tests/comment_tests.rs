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

async fn create_article(app: axum::Router, token: &str, title: &str) -> String {
    let payload = json!({
        "article": {
            "title": title,
            "description": "Test article",
            "body": "Content"
        }
    });

    let response = app
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    body["article"]["slug"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_add_comment_to_article() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;
    let slug = create_article(app.clone(), &token, "Article with Comments").await;

    let payload = json!({
        "comment": {
            "body": "Great article!"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/articles/{}/comments", slug))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["comment"]["body"], "Great article!");
    assert_eq!(body["comment"]["author"]["username"], "user");
    assert!(body["comment"]["id"].is_number());
}

#[tokio::test]
async fn test_add_comment_without_authentication_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;
    let slug = create_article(app.clone(), &token, "Article Without Auth Comments").await;

    let payload = json!({
        "comment": {
            "body": "This should fail"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/articles/{}/comments", slug))
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_add_comment_to_nonexistent_article_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;

    let payload = json!({
        "comment": {
            "body": "Comment on nothing"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles/nonexistent-slug/comments")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_comments_for_article() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;
    let slug = create_article(app.clone(), &token, "Article with Multiple Comments").await;

    let comment1 = json!({"comment": {"body": "First comment"}});
    let comment2 = json!({"comment": {"body": "Second comment"}});

    for payload in [comment1, comment2] {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/articles/{}/comments", slug))
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
                .uri(format!("/api/articles/{}/comments", slug))
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

    assert_eq!(body["comments"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_comments_for_nonexistent_article_fails() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/nonexistent-slug/comments")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_comment_by_author() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;
    let slug = create_article(app.clone(), &token, "Article to Delete Comment From").await;

    let payload = json!({
        "comment": {
            "body": "This will be deleted"
        }
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/articles/{}/comments", slug))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let comment_id = body["comment"]["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/articles/{}/comments/{}", slug, comment_id))
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_delete_comment_by_non_author_fails() {
    let app = common::create_test_app().await;
    let author_token = register_user(app.clone(), "author", "author@example.com", "password123").await;
    let other_token = register_user(app.clone(), "other", "other@example.com", "password123").await;
    let slug = create_article(app.clone(), &author_token, "Protected Comment Article").await;

    let payload = json!({
        "comment": {
            "body": "Cannot be deleted by others"
        }
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/articles/{}/comments", slug))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", author_token))
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let comment_id = body["comment"]["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/articles/{}/comments/{}", slug, comment_id))
                .header("authorization", format!("Bearer {}", other_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_comment_without_authentication_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "user", "user@example.com", "password123").await;
    let slug = create_article(app.clone(), &token, "Article for Unauth Comment Delete").await;

    let payload = json!({
        "comment": {
            "body": "Protected comment"
        }
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/articles/{}/comments", slug))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let comment_id = body["comment"]["id"].as_i64().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/articles/{}/comments/{}", slug, comment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
