use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Used to create a new dog.
#[derive(Deserialize)]
pub struct NewDog {
    pub name: String,
}

/// Represents a complete dog.
#[derive(Serialize, Deserialize)]
pub struct Dog {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}

/// Create the routing configuration for our app.
pub fn app() -> Router {
    Router::new().route(
        "/v1/dogs",
        post(|payload: Json<NewDog>| async {
            Json(Dog {
                id: Uuid::new_v4(),
                name: payload.0.name,
                completed: false,
            })
        }),
    )
}
