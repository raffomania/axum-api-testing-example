use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Used to create a new dog
#[derive(Deserialize)]
pub struct NewDog {
    pub name: String,
}

/// Represents a complete dog
#[derive(Serialize, Deserialize)]
pub struct Dog {
    pub id: Uuid,
    pub name: String,
    pub completed: bool,
}
