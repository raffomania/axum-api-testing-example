pub mod types;

use axum::{routing::post, Json, Router};
use types::{Dog, NewDog};
use uuid::Uuid;

pub fn app() -> Router {
    Router::new().route(
        "/v1/dogs",
        post(|payload: Json<NewDog>| async move {
            Json(Dog {
                id: Uuid::new_v4(),
                name: payload.0.name,
                completed: false,
            })
        }),
    )
}
