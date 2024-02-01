use chrono::Utc;
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Itinerary {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub user_id: usize,
    pub created_at: DateTime<Utc>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Itinerary {
    pub fn new(
        id: usize,
        name: String,
        description: String,
        user_id: usize,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItineraryStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItineraryShareType {
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItineraryShare {
    pub id: i32,
    pub itinerary_id: i32,
    pub user_id: i32,
    pub share_type: ItineraryShareType,
    pub share_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItineraryItem {
    pub id: i32,
    pub name: String,
    pub itinerary_id: i32,
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

impl Activity {
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

#[derive(Clone, Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flight {
    pub id: usize,
    pub itinerary_id: usize,
    pub airline: String,
    pub confirmation_code: String,
    pub departure_airport: String,
    pub departure_time: DateTime<Utc>,
    pub arrival_airport: String,
    pub arrival_time: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Flight {
    pub fn new(
        id: usize,
        itinerary_id: usize,
        airline: String,
        confirmation_code: String,
        departure_airport: String,
        departure_time: DateTime<Utc>,
        arrival_airport: String,
        arrival_time: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            itinerary_id,
            airline,
            confirmation_code,
            departure_airport,
            departure_time,
            arrival_airport,
            arrival_time,
            created_at,
            updated_at,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TravelLeg {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub tz_start: String,
    pub tz_end: String,
}

impl TravelLeg {
    pub fn new(
        id: i32,
        from: i32,
        to: i32,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        tz_start: String,
        tz_end: String,
    ) -> Self {
        Self {
            id,
            from,
            to,
            start,
            end,
            tz_start,
            tz_end,
        }
    }
}
