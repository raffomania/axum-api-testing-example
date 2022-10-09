mod testing_utilities;

pub use testing_utilities::*;

use rstest::rstest;
use serde::de::DeserializeOwned;

#[rstest]
#[case::todos(TestingApp::new().todos())]
#[tokio::test]
async fn is_versioned<'app, Response: DeserializeOwned>(#[case] endpoint: ApiEndpoint<Response>) {
    assert!(endpoint.uri.starts_with("/v1/"));
}
