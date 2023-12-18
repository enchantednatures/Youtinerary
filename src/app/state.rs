use chrono::{DateTime, NaiveDate, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::models::user::{Itinerary, ItineraryShareType};
use crate::models::{ItineraryItem, ItineraryShare, ItineraryStatus, TravelLeg};

pub const STORAGE_KEY: &str = "youtinerary-itineraries";

#[derive(Clone, Debug, Default)]
pub struct LoggedInUser {
    pub id: usize,
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
    pub fn iter(&self) -> impl Iterator<Item = Itinerary> + '_ {
        self.itineraries
            .0
            .iter()
            .map(|(id, value)| (*id, value).into())
    }

    pub fn get_itinerary(&self, id: usize) -> Option<Itinerary> {
        if let Some(itinerary) = self.itineraries.0.get(&id) {
            return Some((id, itinerary).into());
        }
        None
    }

    pub fn add(&mut self, itinerary: CreateItieraryRequest) {
        let storage = window()
            .local_storage()
            .expect("couldn't get localStorage")
            .unwrap();

        let highest_key = self.itineraries.0.keys().max().unwrap_or(&1usize) + 1;

        self.itineraries.0.insert(
            highest_key,
            FullItinerary::new(
                itinerary.name.get(),
                itinerary.description.get(),
                self.user.as_ref().expect("User not logged in").id,
                chrono::Utc::now(),
                itinerary.start_date.get(),
                itinerary.end_date.get(),
                ItineraryStatus::Draft,
                vec![],
                vec![],
                vec![],
            ),
        );
        let json =
            serde_json::to_string(&self.itineraries.0).expect("couldn't serialize Itineraries");
        if storage.set_item(STORAGE_KEY, &json).is_err() {
            log::error!("error while trying to set item in localStorage");
        }
    }
}

pub type GlobalStateSignal = RwSignal<GlobalState>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShareItineraryRequest {
    pub itinerary_id: i32,
    pub user_id: i32,
    pub share_type: ItineraryShareType,
    pub share_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateItineraryItemRequest {
    pub name: String,
    pub itinerary_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateStayRequest {
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateActivityRequest {
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTravelLegRequest {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub tz_start: String,
    pub tz_end: String,
}

trait ItineraryStorage {
    fn create(&mut self, itinerary: Itinerary);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItieraryRequest {
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub start_date: RwSignal<NaiveDate>,
    pub end_date: RwSignal<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct Itineraries(HashMap<usize, FullItinerary>);

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
                        serde_json::from_str::<HashMap<usize, FullItinerary>>(&value).ok()
                    })
            })
            .unwrap_or_default();
        Self(itineraries_from_storage)
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(&self.0).expect("couldn't serialize Itineraries")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FullItinerary {
    pub name: String,
    pub description: String,
    pub user_id: usize,
    pub created_at: DateTime<Utc>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: ItineraryStatus,
    pub items: Vec<ItineraryItem>,
    pub shares: Vec<ItineraryShare>,
    pub travel_legs: Vec<TravelLeg>,
}

impl FullItinerary {
    fn new(
        name: String,
        description: String,
        user_id: usize,
        created_at: DateTime<Utc>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        status: ItineraryStatus,
        items: Vec<ItineraryItem>,
        shares: Vec<ItineraryShare>,
        travel_legs: Vec<TravelLeg>,
    ) -> Self {
        Self {
            name,
            description,
            user_id,
            created_at,
            start_date,
            end_date,
            status,
            items,
            shares,
            travel_legs,
        }
    }
}

impl From<(usize, &FullItinerary)> for Itinerary {
    fn from((id, value): (usize, &FullItinerary)) -> Self {
        Self {
            id,
            name: value.name.clone(),
            description: value.description.clone(),
            user_id: value.user_id,
            created_at: value.created_at,
            start_date: value.start_date,
            end_date: value.end_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItineraryViewModel {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl ItineraryViewModel {
    pub fn new(
        id: usize,
        name: String,
        description: String,
        user_id: i32,
        created_at: DateTime<Utc>,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            id,
            name,
            description,
            user_id,
            created_at,
            start_date,
            end_date,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItineraryStay {
    pub itinerary_id: i32,
    pub stay_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stay {
    pub id: i32,
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

impl Stay {
    pub fn new(
        id: i32,
        summary: i32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        location: String,
        notes: String,
    ) -> Self {
        Self {
            id,
            summary,
            start_date,
            end_date,
            location,
            notes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Activity {
    pub id: i32,
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TravelLegType {
    Flight,
    Train,
    Bus,
    Car,
    Ferry,
    Other,
}
