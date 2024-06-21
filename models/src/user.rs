use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: usize,
    pub theme: Theme,
}
