use anyhow::Result;
use api_core::User;
use api_core::error_handling::AppError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use tracing::Instrument;

#[tracing::instrument(name = "Create Itinerary", skip(db))]
pub async fn create_itinerary(
    State(db): State<PgPool>,
    user: User,
    Json(create_itinerary): Json<CreateItineraryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let itinerary_id = db
        .create_itinerary((user.id, create_itinerary).into())
        .await?;
    Ok((
        StatusCode::CREATED,
        format!("/itineraries/{}", itinerary_id),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItineraryRequest {
    name: String,
}

impl From<(i32, CreateItineraryRequest)> for InsertItinerary {
    fn from(val: (i32, CreateItineraryRequest)) -> Self {
        InsertItinerary {
            user_id: val.0,
            name: val.1.name,
        }
    }
}

struct InsertItinerary {
    user_id: i32,
    name: String,
}

trait CreateItineraryRespository {
    async fn create_itinerary(&self, create_itinerary: InsertItinerary) -> Result<i32>;
}

impl CreateItineraryRespository for PgPool {
    #[tracing::instrument(name = "Insert Itinerary into Database", skip(self, create_itinerary))]
    async fn create_itinerary(&self, create_itinerary: InsertItinerary) -> Result<i32> {
        let inserted = sqlx::query!(
            r#"
            INSERT INTO itineraries (user_id, name)
            VALUES ($1, $2)
            RETURNING itinerary_id
            "#,
            create_itinerary.user_id as i32,
            create_itinerary.name,
        )
        .fetch_one(self)
        .in_current_span()
        .await?;

        Ok(inserted.itinerary_id as i32)
    }
}
