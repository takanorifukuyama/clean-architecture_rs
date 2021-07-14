use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::UserId;

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct User {
    id: UserId,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: UserId::default(),
        }
    }

    pub fn rebuild(id: UserId) -> Self {
        Self { id }
    }
}
