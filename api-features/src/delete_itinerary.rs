use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

#[tracing::instrument(name = "Delete Itinerary", skip(_db))]
pub async fn delete_itinerary(State(_db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "put_itinerary")
}
