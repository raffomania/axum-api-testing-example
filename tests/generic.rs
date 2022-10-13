use hyper::StatusCode;
use rstest::rstest;
use rstest_reuse::{self, *};
use serde_json::json;

mod testing_utilities;

pub use testing_utilities::*;

/// A rstest template with all the cases for our different API endpoints.
/// We can use this so we don't have to repeat the case definitions for every generic test we write
/// below.
#[template]
#[rstest]
#[case::dogs(TestingApp::new().dogs())]
fn generic_test<Response: DeserializeOwned>(#[case] builder: RequestBuilder<Response>) {}

/// Check that all our API endpoint URLs are versioned by insisting that they begin with "/v1/".
#[apply(generic_test)]
#[tokio::test]
async fn is_versioned<Response>(#[case] endpoint: RequestBuilder<Response>) {
    assert!(endpoint.base_url.starts_with("/v1/"));
}

/// Check that sending an empty body will result in an error that is formatted correctly.
#[apply(generic_test)]
#[tokio::test]
async fn empty_body_returns_error<Response>(#[case] endpoint: RequestBuilder<Response>) {
    let StringBody(error) = endpoint
        .expect_error(StatusCode::UNPROCESSABLE_ENTITY)
        .create(&json!({}))
        .await;

    assert!(error.starts_with("Failed to deserialize the JSON body"));
}
