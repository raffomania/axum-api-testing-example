use axum::{
    body::{Body, Bytes},
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use tower::{Service, ServiceExt};

use rust_end_to_end_testing_example::{app, Dog};

/// This is the entry point for our tests.
/// Most tests will start out by calling [TestingApp::new].
/// With that, they can use methods like `dogs()` to obtain a request builder for a specific
/// endpoint.
pub struct TestingApp {
    pub router: axum::Router,
}

impl TestingApp {
    pub fn new() -> TestingApp {
        TestingApp { router: app() }
    }

    pub fn dogs(self) -> RequestBuilder<Dog> {
        RequestBuilder::new(self.router, "/v1/dogs".to_string())
    }
}

pub struct RequestBuilder<Response> {
    /// This is only owned to simplify the demo code.
    /// A real-world application would use a mutable borrow here to enable tests to send multiple
    /// requests to the same backend.
    pub router: axum::Router,
    /// This is the URL of the endpoint we want to talk to, e.g. "/v1/dogs".
    pub base_url: String,
    /// This is the HTTP status that we expect the backend to return.
    /// If it returns a different status, we'll panic.
    pub expected_status: StatusCode,
    /// This is needed so we can use the `Response` type as a generic type parameter for the
    /// [RequestBuilder] struct. Since that struct will never actually contain a value of type
    /// `Response`, we use [PhantomData] instead.
    pub response_type: PhantomData<Response>,
}

impl<Response> RequestBuilder<Response> {
    /// Create a new request builder to dispatch a request to the given URL.
    pub fn new(router: axum::Router, base_url: String) -> RequestBuilder<Response> {
        RequestBuilder {
            router,
            base_url,
            response_type: PhantomData,
            expected_status: StatusCode::OK,
        }
    }

    /// Send a POST request for creating an entity.
    pub async fn create<Input>(mut self, input: &Input) -> Response
    where
        Input: Serialize,
        Response: FromBody,
    {
        let request = Request::builder()
            .method(http::Method::POST)
            .uri(&self.base_url)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&input).unwrap()))
            .unwrap();

        let response = ServiceExt::<Request<Body>>::ready(&mut self.router)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), self.expected_status);

        let body = response.into_body().collect().await.unwrap().to_bytes();

        FromBody::from_body(body)
    }

    /// Make this request builder expect an error response instead of a successful one.
    /// To do this, we set a new status code to expect and  change the `Response` type to
    /// [StringBody]. This will make methods like `create` skip the JSON serialization and just
    /// return the response body as a string.
    pub fn expect_error(self, expected_status: StatusCode) -> RequestBuilder<StringBody> {
        RequestBuilder {
            router: self.router,
            base_url: self.base_url,
            expected_status,
            response_type: PhantomData,
        }
    }
}

/// Our backend can return two different kinds of bodies: JSON values and strings.
/// For JSON bodies, we want to deserialize them before returning them from the request builder.
/// For strings, we just want to pass them through.
/// To do this, we add this trait that specifies how to convert any response body into the
/// `Response` type our request builder wants to return.
pub trait FromBody {
    fn from_body(input: Bytes) -> Self;
}

/// This is the normal case, when our backend returns a JSON body.
/// We implement the [FromBody] trait for any type that can be deserialized from a JSON body.
impl<Response> FromBody for Response
where
    Response: DeserializeOwned,
{
    fn from_body(input: Bytes) -> Response {
        serde_json::from_slice(&input).unwrap()
    }
}

/// When there's an error, our backend will return a plain string.
/// In the request builder, we indicate this by changing the `Response` type to [StringBody].
/// This makes the request builder use the [FromBody] implementation
/// below instead of trying to deserialize a JSON value.
/// It would be simpler to just implement [FromBody] for [String], but since [String] implements
/// [serde::Deserialize] as well, that would conflict with the generic implementation above.
/// We get around that by using the newtype pattern to get a type that doesn't implement
/// [serde::Deserialize].
pub struct StringBody(pub String);

impl FromBody for StringBody {
    fn from_body(input: Bytes) -> Self {
        StringBody(String::from_utf8(input.to_vec()).unwrap())
    }
}
