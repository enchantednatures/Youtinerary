pub mod error_handling;
pub mod middlewares;
use axum::extract::FromRef;
pub use middlewares::User;
use oauth2::basic::BasicClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    redis: redis::Client,
    oauth_client: BasicClient,
    reqwest_client: reqwest::Client,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        redis: redis::Client,
        oauth_client: BasicClient,
        reqwest_client: reqwest::Client,
    ) -> Self {
        Self {
            pool,
            redis,
            oauth_client,
            reqwest_client,
        }
    }
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}

impl FromRef<AppState> for reqwest::Client {
    fn from_ref(state: &AppState) -> Self {
        state.reqwest_client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}
