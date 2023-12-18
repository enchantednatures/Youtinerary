use leptos::*;
use std::collections::HashMap;

use crate::models::user::Itinerary;

pub const STORAGE_KEY: &str = "youtinerary-itineraries";

#[derive(Clone, Debug, Default)]
pub struct LoggedInUser {
    pub email: String,
    pub name: String,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Default)]
pub struct GlobalState {
    pub user: Option<LoggedInUser>,
    pub itineraries: Itineraries,
}

impl GlobalState {
    pub fn get_itinerary(&self, id: usize) -> Option<&Itinerary> {
        self.itineraries.0.get(&id)
    }
}

pub type GlobalStateSignal = RwSignal<GlobalState>;

#[derive(Debug, Clone)]
pub struct Itineraries(HashMap<usize, Itinerary>);

impl Default for Itineraries {
    fn default() -> Self {
        Self::new()
    }
}
impl Itineraries {
    pub fn new() -> Self {
        let itineraries_from_storage = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| {
                storage
                    .get_item(STORAGE_KEY)
                    .ok()
                    .flatten()
                    .and_then(|value| {
                        serde_json::from_str::<HashMap<usize, Itinerary>>(&value).ok()
                    })
            })
            .unwrap_or_default();
        Self(itineraries_from_storage)
    }

    pub fn add(&mut self, itinerary: Itinerary) {
        let storage = window()
            .local_storage()
            .expect("couldn't get localStorage")
            .unwrap();
        let highest_key = self.0.keys().max().unwrap_or(&1usize);
        self.0.insert(highest_key + 1, itinerary);
        let json = serde_json::to_string(&self.0).expect("couldn't serialize Itineraries");
        if storage.set_item(STORAGE_KEY, &json).is_err() {
            log::error!("error while trying to set item in localStorage");
        }
    }

    pub fn get(&self) -> impl Iterator<Item = &Itinerary> {
        self.0.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Itinerary)> {
        self.0.iter()
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(&self.0).expect("couldn't serialize Itineraries")
    }
}
