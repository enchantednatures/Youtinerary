mod create_flight;
mod create_itinerary;
mod create_stay;
mod delete_itinerary;
mod get_itineraries;
mod get_itinerary;
mod get_itinerary_stays;
mod post_itinerary_stay;
mod update_itinerary;
use api_core::AppState;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use create_flight::create_flight;
use create_itinerary::create_itinerary;
use delete_itinerary::delete_itinerary;
use get_itineraries::get_itineraries;
use get_itinerary::get_itinerary;
use get_itinerary_stays::get_itinerary_stays;
use post_itinerary_stay::post_itinerary_stay;
use update_itinerary::put_itinerary;

pub fn itineraries_router() -> Router<AppState> {
    Router::new()
        .route("/itineraries", get(get_itineraries).post(create_itinerary))
        .route(
            "/itineraries/:id",
            get(get_itinerary)
                .put(put_itinerary)
                .delete(delete_itinerary),
        )
        .route("/itineraries/:id/flights", post(create_flight))
        .route(
            "/itineraries/:id/stays",
            get(get_itinerary_stays).post(post_itinerary_stay),
        )
}
