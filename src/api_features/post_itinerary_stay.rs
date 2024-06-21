use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

#[tracing::instrument(name = "Post Itinerary Stay", skip(_db))]
pub async fn post_itinerary_stay(
    State(_db): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itinerary_stays")
}
