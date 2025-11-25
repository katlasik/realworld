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
async fn test_create_article_with_valid_data() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let payload = json!({
        "article": {
            "title": "Test Article",
            "description": "This is a test article",
            "body": "Article content goes here",
            "tagList": ["rust", "testing"]
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
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

    assert_eq!(body["article"]["title"], "Test Article");
    assert_eq!(body["article"]["description"], "This is a test article");
    assert_eq!(body["article"]["body"], "Article content goes here");
    assert_eq!(body["article"]["slug"], "test-article");
    assert_eq!(body["article"]["tagList"], json!(["rust", "testing"]));
    assert_eq!(body["article"]["favorited"], false);
    assert_eq!(body["article"]["favoritesCount"], 0);
}

#[tokio::test]
async fn test_create_article_without_authentication_fails() {
    let app = common::create_test_app().await;

    let payload = json!({
        "article": {
            "title": "Test Article",
            "description": "This is a test article",
            "body": "Article content goes here"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_articles_returns_all_articles() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let payload1 = json!({
        "article": {
            "title": "First Article",
            "description": "First test article",
            "body": "Content 1",
            "tagList": ["foo", "bar", "baz"]
        }
    });

    let payload2 = json!({
        "article": {
            "title": "Second Article",
            "description": "Second test article",
            "body": "Content 2",
            "tagList": ["baz"]
        }
    });

    for payload in [payload1, payload2] {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Token {}", token))
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
                .uri("/api/articles")
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

    assert_eq!(body["articlesCount"], 2);
    assert_eq!(body["articles"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_article_by_slug() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Get Me By Slug",
            "description": "Test article to retrieve",
            "body": "Content here"
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/get-me-by-slug")
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

    assert_eq!(body["article"]["title"], "Get Me By Slug");
    assert_eq!(body["article"]["slug"], "get-me-by-slug");
}

#[tokio::test]
async fn test_get_nonexistent_article_returns_404() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/nonexistent-slug")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_article_by_author() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Original Title",
            "description": "Original description",
            "body": "Original body"
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let update_payload = json!({
        "article": {
            "title": "Updated Title",
            "description": "Updated description",
            "body": "Updated body"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/articles/original-title")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["article"]["title"], "Updated Title");
    assert_eq!(body["article"]["description"], "Updated description");
    assert_eq!(body["article"]["body"], "Updated body");
}

#[tokio::test]
async fn test_update_article_to_existing_slug_fails() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Original Title 1",
            "description": "Original description",
            "body": "Original body"
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_payload = json!({
        "article": {
            "title": "Original Title 2",
            "description": "Original description",
            "body": "Original body"
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let update_payload = json!({
        "article": {
            "title": "Original Title 1",
            "description": "Updated description",
            "body": "Updated body"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/articles/original-title-2")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_update_article_by_non_author_fails() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let other_token = register_user(app.clone(), "other", "other@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Authors Article",
            "description": "Only author can update",
            "body": "Content"
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let update_payload = json!({
        "article": {
            "title": "Hacked Title"
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/articles/authors-article")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", other_token))
                .body(Body::from(serde_json::to_string(&update_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_article_by_author() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "To Be Deleted",
            "description": "This will be deleted",
            "body": "Content",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/articles/to-be-deleted")
                .header("authorization", format!("Token {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_delete_article_by_non_author_fails() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let other_token = register_user(app.clone(), "other", "other@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Protected Article",
            "description": "Cannot be deleted by others",
            "body": "Content",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/articles/protected-article")
                .header("authorization", format!("Token {}", other_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_favorite_article() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let user_token = register_user(app.clone(), "user", "user@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Article to Favorite",
            "description": "User will favorite this",
            "body": "Content",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles/article-to-favorite/favorite")
                .header("authorization", format!("Token {}", user_token))
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

    assert_eq!(body["article"]["favorited"], true);
}

#[tokio::test]
async fn test_unfavorite_article() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let user_token = register_user(app.clone(), "user", "user@example.com", "password123").await;

    let create_payload = json!({
        "article": {
            "title": "Article to Unfavorite",
            "description": "User will unfavorite this",
            "body": "Content",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles/article-to-unfavorite/favorite")
                .header("authorization", format!("Token {}", user_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/articles/article-to-unfavorite/favorite")
                .header("authorization", format!("Token {}", user_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_article_feed_for_authenticated_user() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let follower_token = register_user(
        app.clone(),
        "follower",
        "follower@example.com",
        "password123",
    )
    .await;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/author/follow")
                .header("authorization", format!("Token {}", follower_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let create_payload = json!({
        "article": {
            "title": "Feed Article",
            "description": "Should appear in feed",
            "body": "Content",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/feed")
                .header("authorization", format!("Token {}", follower_token))
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

    assert_eq!(body["articlesCount"], 1);
    assert_eq!(body["articles"][0]["title"], "Feed Article");
}

#[tokio::test]
async fn test_get_paginated_article_feed() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let follower_token = register_user(
        app.clone(),
        "follower",
        "follower@example.com",
        "password123",
    )
    .await;

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/profiles/author/follow")
                .header("authorization", format!("Token {}", follower_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    for idx in 0..100 {
        let create_payload = json!({
            "article": {
                "title": format!("Feed Article {}", idx),
                "description": "Should appear in feed",
                "body": "Content",
                "tagList": []
            }
        });

        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Token {}", author_token))
                    .body(Body::from(serde_json::to_string(&create_payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/feed")
                .header("authorization", format!("Token {}", follower_token))
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

    assert_eq!(body["articlesCount"], 100);
    assert_eq!(body["articles"].as_array().unwrap().len(), 50);
}

#[tokio::test]
async fn test_get_feed_without_authentication_fails() {
    let app = common::create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles/feed")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_filter_articles_by_tag() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    let article_with_rust = json!({
        "article": {
            "title": "Rust Article",
            "description": "About Rust",
            "body": "Rust content",
            "tagList": ["rust", "programming"]
        }
    });

    let article_with_python = json!({
        "article": {
            "title": "Python Article",
            "description": "About Python",
            "body": "Python content",
            "tagList": ["python", "programming"]
        }
    });

    let article_with_javascript = json!({
        "article": {
            "title": "JavaScript Article",
            "description": "About JavaScript",
            "body": "JavaScript content",
            "tagList": ["javascript"]
        }
    });

    for payload in [
        article_with_rust,
        article_with_python,
        article_with_javascript,
    ] {
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Token {}", token))
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
                .uri("/api/articles?tag=rust")
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

    assert_eq!(body["articlesCount"], 1);
    assert_eq!(body["articles"][0]["title"], "Rust Article");
    assert!(
        body["articles"][0]["tagList"]
            .as_array()
            .unwrap()
            .contains(&json!("rust"))
    );
}

#[tokio::test]
async fn test_filter_articles_by_author() {
    let app = common::create_test_app().await;
    let author1_token =
        register_user(app.clone(), "author1", "author1@example.com", "password123").await;
    let author2_token =
        register_user(app.clone(), "author2", "author2@example.com", "password123").await;

    let article_by_author1 = json!({
        "article": {
            "title": "Article by Author 1",
            "description": "First author's article",
            "body": "Content by author 1",
            "tagList": []
        }
    });

    let article_by_author2 = json!({
        "article": {
            "title": "Article by Author 2",
            "description": "Second author's article",
            "body": "Content by author 2",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author1_token))
                .body(Body::from(
                    serde_json::to_string(&article_by_author1).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author2_token))
                .body(Body::from(
                    serde_json::to_string(&article_by_author2).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles?author=author1")
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

    assert_eq!(body["articlesCount"], 1);
    assert_eq!(body["articles"][0]["title"], "Article by Author 1");
    assert_eq!(body["articles"][0]["author"]["username"], "author1");
}

#[tokio::test]
async fn test_filter_articles_by_favorited() {
    let app = common::create_test_app().await;
    let author_token =
        register_user(app.clone(), "author", "author@example.com", "password123").await;
    let user1_token = register_user(app.clone(), "user1", "user1@example.com", "password123").await;
    let user2_token = register_user(app.clone(), "user2", "user2@example.com", "password123").await;

    let article1 = json!({
        "article": {
            "title": "Article One",
            "description": "First article",
            "body": "Content 1",
            "tagList": []
        }
    });

    let article2 = json!({
        "article": {
            "title": "Article Two",
            "description": "Second article",
            "body": "Content 2",
            "tagList": []
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&article1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author_token))
                .body(Body::from(serde_json::to_string(&article2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles/article-one/favorite")
                .header("authorization", format!("Token {}", user1_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles/article-two/favorite")
                .header("authorization", format!("Token {}", user2_token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles?favorited=user1")
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

    assert_eq!(body["articlesCount"], 1);
    assert_eq!(body["articles"][0]["title"], "Article One");
    assert_eq!(body["articles"][0]["favoritesCount"], 1);
}

#[tokio::test]
async fn test_paginate_articles_with_limit_and_offset() {
    let app = common::create_test_app().await;
    let token = register_user(app.clone(), "author", "author@example.com", "password123").await;

    for i in 0..10 {
        let article = json!({
            "article": {
                "title": format!("Article {}", i),
                "description": format!("Description {}", i),
                "body": format!("Body {}", i),
                "tagList": []
            }
        });

        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/articles")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Token {}", token))
                    .body(Body::from(serde_json::to_string(&article).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles?limit=5&offset=0")
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

    assert_eq!(body["articlesCount"], 10);
    assert_eq!(body["articles"].as_array().unwrap().len(), 5);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles?limit=3&offset=5")
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

    assert_eq!(body["articlesCount"], 10);
    assert_eq!(body["articles"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn test_combine_multiple_filters() {
    let app = common::create_test_app().await;
    let author1_token =
        register_user(app.clone(), "author1", "author1@example.com", "password123").await;
    let author2_token =
        register_user(app.clone(), "author2", "author2@example.com", "password123").await;

    let rust_article_by_author1 = json!({
        "article": {
            "title": "Rust by Author 1",
            "description": "Rust content by author 1",
            "body": "Content",
            "tagList": ["rust", "programming"]
        }
    });

    let python_article_by_author1 = json!({
        "article": {
            "title": "Python by Author 1",
            "description": "Python content by author 1",
            "body": "Content",
            "tagList": ["python", "programming"]
        }
    });

    let rust_article_by_author2 = json!({
        "article": {
            "title": "Rust by Author 2",
            "description": "Rust content by author 2",
            "body": "Content",
            "tagList": ["rust", "webdev"]
        }
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author1_token))
                .body(Body::from(
                    serde_json::to_string(&rust_article_by_author1).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author1_token))
                .body(Body::from(
                    serde_json::to_string(&python_article_by_author1).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/articles")
                .header("content-type", "application/json")
                .header("authorization", format!("Token {}", author2_token))
                .body(Body::from(
                    serde_json::to_string(&rust_article_by_author2).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/articles?tag=rust&author=author1")
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

    assert_eq!(body["articlesCount"], 1);
    assert_eq!(body["articles"][0]["title"], "Rust by Author 1");
    assert_eq!(body["articles"][0]["author"]["username"], "author1");
    assert!(
        body["articles"][0]["tagList"]
            .as_array()
            .unwrap()
            .contains(&json!("rust"))
    );
}
