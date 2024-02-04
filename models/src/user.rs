

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub theme: Theme,
}

