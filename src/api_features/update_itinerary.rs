use auth::AuthentikUser;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItineraryRequest {
    pub name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[tracing::instrument(name = "Put Itinerary", skip(_db))]
pub async fn put_itinerary(
    State(_db): State<PgPool>,
    user: AuthentikUser,
    Path(id): Path<i32>,
    Json(request): Json<UpdateItineraryRequest>,
) -> impl IntoResponse {
    dbg!(request);
    (StatusCode::NOT_IMPLEMENTED, "put_itinerary")
}
