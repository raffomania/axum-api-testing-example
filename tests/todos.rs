use axum::http::{self, Request, StatusCode};
use hyper::Body;
use rust_end_to_end_testing_example::Dog;
use serde_json::json;
use tower::ServiceExt;

mod testing_utilities;

pub use testing_utilities::*;

#[tokio::test]
async fn create_dog_v1() {
    let dog_request = json!({"name": "Write tests"});

    let app = TestingApp::new();

    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/v1/dogs")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(serde_json::to_vec(&dog_request).unwrap()))
        .unwrap();

    let response = app.router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Check that the response body matches the `Dog` struct
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let dog: Dog = serde_json::from_slice(&body).unwrap();
    // Check that our input was used
    assert_eq!(dog.name, "Write tests");
    // Check that the default is what we expect
    assert_eq!(dog.completed, false);
}

#[tokio::test]
async fn create_dog_v2() {
    let dog_request = json!({"name": "Write tests"});

    let app = TestingApp::new();

    let dog = app.dogs().create(&dog_request).await;

    // Check that our input was used
    assert_eq!(dog.name, "Write tests");
    // Check that the default is what we expect
    assert_eq!(dog.completed, false);
}
