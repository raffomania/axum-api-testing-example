pub mod types;

use axum::{routing::post, Json, Router};
use types::{Todo, TodoRequest};
use uuid::Uuid;

pub fn app() -> Router {
    Router::new().route(
        "/v1/todos",
        post(|payload: Json<TodoRequest>| async move {
            Json(Todo {
                id: Uuid::new_v4(),
                name: payload.0.name,
                completed: false,
            })
        }),
    )
}
