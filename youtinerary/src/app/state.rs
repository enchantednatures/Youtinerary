use leptos::{RwSignal, window};


const STORAGE_KEY: &str = "youtinerary-itineraries";

#[derive(Debug, Clone)]
pub struct Itineraries(Vec<Itinerary>);

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Itinerary {
    pub name: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Itinerary {
    pub fn new(
        name: String,
        description: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            name,
            description,
            start_date,
            end_date,
        }
    }
}

impl From<CreateItieraryRequest> for Itinerary {
    fn from(request: CreateItieraryRequest) -> Self {
        Self::new(request.name.get(), request.description.get(), request.start_date.get(), request.end_date.get())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItieraryRequest {
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub start_date: RwSignal<NaiveDate>,
    pub end_date: RwSignal<NaiveDate>,
}

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
