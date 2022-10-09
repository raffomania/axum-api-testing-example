use axum::http::{self, Request, StatusCode};
use hyper::Body;
use std::marker::PhantomData;
use tower::Service;
use tower::ServiceExt;

use serde::{de::DeserializeOwned, Serialize};

pub struct ApiEndpoint<Response: DeserializeOwned> {
    /// This is only owned to simplify the demo code.
    /// A real-world application would use a mutable borrow here to enable tests to send multiple requests to the same backend.
    pub router: axum::Router,
    pub uri: String,
    pub response_type: PhantomData<Response>,
}

impl<Response: DeserializeOwned> ApiEndpoint<Response> {
    pub fn new(router: axum::Router, uri: String) -> ApiEndpoint<Response> {
        ApiEndpoint {
            router,
            uri,
            response_type: PhantomData,
        }
    }

    pub async fn create<Input: Serialize>(mut self, body: &Input) -> Response {
        let response = self
            .router
            .ready()
            .await
            .unwrap()
            .call(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(&self.uri)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        serde_json::from_slice(&body).unwrap()
    }
}
