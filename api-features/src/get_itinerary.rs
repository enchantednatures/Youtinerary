use anyhow::Result;
use auth::AuthentikUser;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::Error;
use sqlx::PgPool;
use uuid::Uuid;

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
    user: AuthentikUser,
    Path(itinerary_id): Path<i32>,
    State(db): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    match db.get_itinerary(user.sub, itinerary_id).await {
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
        user_id: Uuid,
        itinerary_id: i32,
    ) -> Result<Itinerary, GetItineraryError>;
}

enum GetItineraryError {
    UnableToFindItinerary,
}
impl From<Error> for GetItineraryError {
    fn from(_value: Error) -> Self {
        Self::UnableToFindItinerary
    }
}

impl GetItineraryRespository for PgPool {
    async fn get_itinerary(
        &self,
        user_id: Uuid,
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
    pub user_id: Uuid,
}
