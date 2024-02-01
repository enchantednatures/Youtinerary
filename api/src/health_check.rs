use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;
use axum::Json;
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

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthStatus::new()))
}
