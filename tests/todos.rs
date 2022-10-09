use axum::http::{self, Request, StatusCode};
use end_to_end_testing_rust_demo::types::Todo;
use hyper::Body;
use serde_json::json;
use tower::ServiceExt;

mod testing_utilities;

pub use testing_utilities::*;

#[tokio::test]
async fn create_todo_v1() {
    let todo_request = json!({"name": "Write tests"});

    let app = TestingApp::new();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/v1/todos")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&todo_request).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Check that the response body matches the `Todo` struct
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let todo: Todo = serde_json::from_slice(&body).unwrap();
    // Check that our input was used
    assert_eq!(todo.name, "Write tests");
    // Check that the default is what we expect
    assert_eq!(todo.completed, false);
}

#[tokio::test]
async fn create_todo_v2() {
    let todo_request = json!({"name": "Write tests"});

    let app = TestingApp::new();

    let todo = app.todos().create(&todo_request).await;

    // Check that our input was used
    assert_eq!(todo.name, "Write tests");
    // Check that the default is what we expect
    assert_eq!(todo.completed, false);
}
