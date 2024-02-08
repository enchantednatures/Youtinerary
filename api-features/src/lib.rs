mod create_flight;
mod create_itinerary;
mod create_stay;
mod delete_itinerary;
mod get_itineraries;
mod get_itinerary;
mod update_itinerary;

use api_core::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use create_flight::create_flight;
use create_itinerary::create_itinerary;
use delete_itinerary::delete_itinerary;
use get_itineraries::get_itineraries;
use get_itinerary::get_itinerary;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::NaiveDate;
use sqlx::PgPool;
use update_itinerary::put_itinerary;

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

#[tracing::instrument(name = "Get Itinerary Stays", skip(_db))]
pub async fn get_itinerary_stays(
    State(_db): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itinerary_stays")
}

#[tracing::instrument(name = "Post Itinerary Stay", skip(_db))]
pub async fn post_itinerary_stay(
    State(_db): State<PgPool>,
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
