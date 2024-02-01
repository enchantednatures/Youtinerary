use axum::extract::{Path, State};

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error_handling::AppError;

#[tracing::instrument(name = "Create Flight", skip(db))]
pub async fn create_flight(
    State(db): State<PgPool>,
    Path(itinerary_id): Path<i32>,
    Json(create_flight): Json<CreateFlightRequest>,
) -> Result<impl IntoResponse, AppError> {
    let created_id = db
        .create_flight((itinerary_id, create_flight).into())
        .await?;
    Ok((StatusCode::CREATED, format!("/itineraries/{}/flights/{}", itinerary_id, created_id)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFlightRequest {
    pub(crate) airline: String,
    pub(crate) confirmation_code: String,
    pub(crate) departure_time: DateTime<Utc>,
    pub(crate) arrival_time: DateTime<Utc>,
    pub(crate) notes: String,
}

impl Into<InsertFlight> for (i32, CreateFlightRequest) {
    fn into(self) -> InsertFlight {
        InsertFlight {
            itinerary_id: self.0,
            airline: self.1.airline,
            confirmation_code: self.1.confirmation_code,
            departure_time: self.1.departure_time,
            arrival_time: self.1.arrival_time,
            notes: self.1.notes,
        }
    }
}

struct InsertFlight {
    itinerary_id: i32,
    airline: String,
    confirmation_code: String,
    departure_time: DateTime<Utc>,
    arrival_time: DateTime<Utc>,
    notes: String,
}

trait CreateFlightRespository {
    async fn create_flight(&self, create_flight: InsertFlight) -> Result<i32>;
}

impl CreateFlightRespository for PgPool {
    async fn create_flight(&self, create_flight: InsertFlight) -> Result<i32> {
        let created_id = sqlx::query!(
            r#"
            insert into flights (airline, confirmation_code, departure_time, arrival_time, notes)
            values ($1, $2, $3, $4, $5)
            returning id
            "#,
            create_flight.airline,
            create_flight.confirmation_code,
            create_flight.departure_time,
            create_flight.arrival_time,
            create_flight.notes,
        )
        .fetch_one(self)
        .await?;

        Ok(created_id.id as i32)
    }
}
