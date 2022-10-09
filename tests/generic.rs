mod testing_utilities;

pub use testing_utilities::*;

use rstest::rstest;

#[rstest]
#[case::todos(TestingApp::new().todos())]
#[tokio::test]
async fn is_versioned<Response>(#[case] endpoint: ApiEndpoint<Response>) {
    assert!(endpoint.uri.starts_with("/v1/"));
}
