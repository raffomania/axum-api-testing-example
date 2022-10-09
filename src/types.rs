use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Used to create a new todo
#[derive(Deserialize)]
pub struct TodoRequest {
    pub name: String,
}

/// Represents a complete todo
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}
