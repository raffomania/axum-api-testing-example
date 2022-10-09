use axum::Router;

use end_to_end_testing_rust_demo::{app, types::Todo};

use super::ApiEndpoint;

pub struct TestingApp {
    pub router: Router,
}

impl TestingApp {
    pub fn new() -> TestingApp {
        TestingApp { router: app() }
    }

    pub fn todos(self) -> ApiEndpoint<Todo> {
        ApiEndpoint::new(self.router, "/v1/todos".to_string())
    }
}
