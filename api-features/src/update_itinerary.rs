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

#[tracing::instrument(name = "Put Itinerary", skip(_db))]
pub async fn put_itinerary(State(_db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "put_itinerary")
}
