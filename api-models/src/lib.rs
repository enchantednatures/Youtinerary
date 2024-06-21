use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::NaiveDate;
use sqlx::types::chrono::Utc;
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "itinerary_status", rename_all = "lowercase")]
pub enum ItineraryStatus {
    Draft,
    Published,
    Archived,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "itinerary_share_type", rename_all = "lowercase")]
pub enum ItineraryShareType {
    Editor,
    Viewer,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ItineraryShare {
    pub id: i32,
    pub itinerary_id: i32,
    pub user_id: i32,
    pub share_type: ItineraryShareType,
    pub share_message: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ItineraryItem {
    pub id: i32,
    pub name: String,
    pub itinerary_id: i32,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
pub struct ItineraryStay {
    pub itinerary_id: i32,
    pub stay_id: i32,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Stay {
    pub id: i32,
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Activity {
    pub id: i32,
    pub summary: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub location: String,
    pub notes: String,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "travel_leg_type", rename_all = "lowercase")]
pub enum TravelLegType {
    Flight,
    Train,
    Bus,
    Car,
    Ferry,
    Other,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct TravelLeg {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub tz_start: String,
    pub tz_end: String,
}
