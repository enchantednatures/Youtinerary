pub mod error_handling;
mod features;
mod health_check;
mod middlewares;
mod models;
use std::net::SocketAddr;

use anyhow::Context;
use anyhow::Result;
use axum::extract::FromRef;
use axum::{routing::get, Router};
pub use health_check::*;
pub use models::*;
use oauth2::basic::BasicClient;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

mod configuration;

use configuration::Settings;
use auth::authorize;
use auth::login_authorized;
use auth::protected;

use self::features::itineraries_router;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    redis: redis::Client,
    oauth_client: BasicClient,
    reqwest_client: reqwest::Client,
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

async fn connect_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}

#[tokio::main]
async fn main() -> Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("youtinerary".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let settings = Settings::load_config().unwrap();

    let pool: PgPool = connect_database(&std::env::var("DATABASE_URL")?).await;
    let redis = redis::Client::open(std::env::var("REDIS_URL")?).unwrap();
    sqlx::migrate!().run(&pool).await?;

    let state = AppState {
        pool,
        redis,
        oauth_client: settings.auth_settings.try_into()?,
        reqwest_client: reqwest::Client::new(),
    };

    let listener = tokio::net::TcpListener::bind(SocketAddr::from((
        settings.app_settings.addr,
        settings.app_settings.port,
    )))
    .await
    .context("failed to bind TcpListener")
    .unwrap();


    let router = Router::new()
        .route("/", get(health_check))
        .route("/health_check", get(health_check))
        .route("/protected", get(protected))
        .route("/authorize", get(authorize))
        .route("/authorized", get(login_authorized))
        .nest("/api/v0", itineraries_router())
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        // .route("", get(retrieve))
        .with_state(state);

    axum::serve(listener, router).await.unwrap();
    Ok(())
}
