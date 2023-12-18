use leptos::*;

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

pub type GlobalStateSignal = RwSignal<GlobalState>;

#[derive(Debug, Clone)]
pub struct Itineraries(Vec<Itinerary>);

impl IntoIterator for Itineraries {
    type Item = Itinerary;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Default for Itineraries {
    fn default() -> Self {
        Self::new()
    }
}
impl Itineraries {
    pub fn new() -> Self {
        let itineraries_from_storage =
            window()
                .local_storage()
                .ok()
                .flatten()
                .and_then(|storage| {
                    storage
                        .get_item(STORAGE_KEY)
                        .ok()
                        .flatten()
                        .and_then(|value| serde_json::from_str::<Vec<Itinerary>>(&value).ok())
                })
                .unwrap_or_default();
        Self(itineraries_from_storage)
    }

    pub fn add(&mut self, itinerary: Itinerary) {
        let storage = window()
            .local_storage()
            .expect("couldn't get localStorage")
            .unwrap();
        self.0.push(itinerary);
        let json = serde_json::to_string(&self.0).expect("couldn't serialize Todos");
        if storage.set_item(STORAGE_KEY, &json).is_err() {
            log::error!("error while trying to set item in localStorage");
        }
    }

    pub fn get(&self) -> &[Itinerary] {
        self.0.as_slice()
    }
}


