use axum::http::{self, Request, StatusCode};
use hyper::Body;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use tower::Service;
use tower::ServiceExt;

use rust_end_to_end_testing_example::{app, types::Dog};

pub struct TestingApp {
    pub router: axum::Router,
}

impl TestingApp {
    pub fn new() -> TestingApp {
        TestingApp { router: app() }
    }

    pub fn dogs(self) -> RequestBuilder<Dog> {
        RequestBuilder {
            router: self.router,
            base_url: "/v1/dogs".to_string(),
            expected_status: StatusCode::OK,
            response_type: PhantomData,
        }
    }
}

pub struct RequestBuilder<Response> {
    /// This is only owned to simplify the demo code.
    /// A real-world application would use a mutable borrow here to enable tests to send multiple requests to the same backend.
    pub router: axum::Router,
    pub base_url: String,
    pub expected_status: StatusCode,
    pub response_type: PhantomData<Response>,
}

impl<Response: DeserializeOwned> RequestBuilder<Response> {
    pub fn new(router: axum::Router, uri: String) -> RequestBuilder<Response> {
        RequestBuilder {
            router,
            base_url: uri,
            response_type: PhantomData,
            expected_status: StatusCode::OK,
        }
    }

    pub async fn create<Input: Serialize>(mut self, input: &Input) -> Response {
        let response = self
            .router
            .ready()
            .await
            .unwrap()
            .call(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(&self.base_url)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&input).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), self.expected_status);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        serde_json::from_slice(&body).unwrap()
    }
}
