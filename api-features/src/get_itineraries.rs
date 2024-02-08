use anyhow::Result;
use api_core::error_handling::AppError;
use auth::AuthentikUser;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct IntinerarySummaryView<'a> {
    id: i32,
    name: &'a str,
}

impl<'a> From<&'a Itinerary> for IntinerarySummaryView<'a> {
    fn from(value: &'a Itinerary) -> Self {
        Self {
            id: value.id,
            name: &value.name,
        }
    }
}

#[tracing::instrument(name = "Get Itineraries", skip(_db))]
pub async fn get_itineraries(
    user: AuthentikUser,
    State(_db): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    // let itineraries: Vec<IntinerarySummaryView> = db
    //     .get_itineraries(user.id)
    //     .await?
    //     .into_iter()
    //     .map(|x| x.into())
    //     .collect();

    let itineraries: Vec<IntinerarySummaryView> = vec![];

    // let itineraries: Vec<IntinerarySummaryView> = db
    //     .get_itineraries(user.id);

    Ok((StatusCode::OK, Json(itineraries)))
}

trait GetItineraryRespository {
    async fn get_itineraries(&self, user_id: Uuid) -> Result<Vec<Itinerary>>;
}

impl GetItineraryRespository for PgPool {
    async fn get_itineraries(&self, user_id: Uuid) -> Result<Vec<Itinerary>> {
        let itineraries = sqlx::query_as!(
            Itinerary,
            r#"
                select 
                    itinerary_id as "id!",
                    name,
                    user_id
                from itineraries
                where user_id = $1
            "#,
            user_id
        )
        .fetch_all(self)
        .await?;

        Ok(itineraries)
    }
}

pub struct Itinerary {
    pub id: i32,
    pub name: String,
    pub user_id: Uuid,
}
