use anyhow::Result;
use api_core::error_handling::AppError;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;

#[tracing::instrument(name = "Delete Itineraries", skip(_db))]
pub fn delete_itinerary(State(_db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "delete_itinerary")
}
