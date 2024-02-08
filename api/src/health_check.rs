use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum HealthStatusEnum {
    Ok,
    Error,
}

#[derive(Deserialize, Serialize)]
pub struct HealthStatus {
    status: HealthStatusEnum,
}

impl HealthStatus {
    pub(crate) fn new() -> Self {
        HealthStatus {
            status: HealthStatusEnum::Ok,
        }
    }
}

#[tracing::instrument(name = "Health Check")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthStatus::new()))
}
