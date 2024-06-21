use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

#[tracing::instrument(name = "Get Itinerary Stays", skip(_db))]
pub async fn get_itinerary_stays(
    State(_db): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itinerary_stays")
}
