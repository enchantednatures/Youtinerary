mod create_flight;
mod create_itinerary;
mod create_stay;
mod create_user;
mod get_itineraries;
mod get_itinerary;

use create_flight::create_flight;
use create_itinerary::create_itinerary;
use get_itineraries::get_itineraries;
use get_itinerary::get_itinerary;

use axum::extract::{Path, State};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDate, PgPool};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItineraryRequest {
    pub name: String,
    pub user_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItineraryRequest {
    pub name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteItineraryRequest {
    pub id: i32,
}

#[tracing::instrument(name = "Put Itinerary", skip(db))]
pub async fn put_itinerary(State(db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "put_itinerary")
}

#[tracing::instrument(name = "Delete Itineraries", skip(db))]
pub fn delete_itinerary(State(db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "delete_itinerary")
}

#[tracing::instrument(name = "Get Itinerary Stays", skip(db))]

pub async fn get_itinerary_stays(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itinerary_stays")
}

#[tracing::instrument(name = "Post Itinerary Stay", skip(db))]
pub async fn post_itinerary_stay(
    State(db): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itinerary_stays")
}

pub fn itineraries_router() -> Router<AppState> {
    Router::new()
        .route("/itineraries", get(get_itineraries).post(create_itinerary))
        .route("/itineraries/:id", get(get_itinerary).put(put_itinerary))
        .route("/itineraries/:id/flights", post(create_flight))
        .route(
            "/itineraries/:id/stays",
            get(get_itinerary_stays).post(post_itinerary_stay),
        )
}
