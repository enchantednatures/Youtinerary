use axum::extract::{Path, State};
use axum::Json;

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::json;
use sqlx::{Error, PgPool};

use crate::User;
use crate::error_handling::AppError;

#[derive(Serialize)]
pub struct ItineraryViewModel {
    pub id: i32,
    pub name: String,
}
impl From<Itinerary> for ItineraryViewModel {
    fn from(value: Itinerary) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
struct ErrorMessage {
    message: String,
}

#[tracing::instrument(name = "Get Itinerary", skip(db))]
pub async fn get_itinerary(
    user: User,
    Path(itinerary_id): Path<i32>,
    State(db): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    match db.get_itinerary(user.id, itinerary_id).await {
        Ok(itinerary) => {
            let itinerary_view_model = ItineraryViewModel::from(itinerary);
            return Ok((StatusCode::OK, Json(itinerary_view_model)));
        }
        Err(error) => match error {
            GetItineraryError::UnableToFindItinerary => Err(StatusCode::NOT_FOUND),
        },
    }
}

trait GetItineraryRespository {
    async fn get_itinerary(
        &self,
        user_id: i32,
        itinerary_id: i32,
    ) -> Result<Itinerary, GetItineraryError>;
}

enum GetItineraryError {
    UnableToFindItinerary,
}
impl From<Error> for GetItineraryError {
    fn from(value: Error) -> Self {
        Self::UnableToFindItinerary
    }
}

impl GetItineraryRespository for PgPool {
    async fn get_itinerary(
        &self,
        user_id: i32,
        itinerary_id: i32,
    ) -> Result<Itinerary, GetItineraryError> {
        let itinerary = sqlx::query_as!(
            Itinerary,
            r#"
                select 
                    itinerary_id as "id!",
                    name,
                    user_id
                from itineraries
                where user_id = $1
                    and itinerary_id = $2
                limit 1
            "#,
            user_id,
            itinerary_id
        )
        .fetch_one(self)
        .await?;

        Ok(itinerary)
    }
}

pub struct Itinerary {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
