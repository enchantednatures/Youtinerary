mod configuration;
pub mod error_handling;
mod features;
mod health_check;
mod middlewares;
mod models;
use self::features::itineraries_router;
use anyhow::Context;
use anyhow::Result;
use auth::authorize;
use auth::login_authorized;
use auth::protected;
use axum::error_handling::HandleErrorLayer;
use axum::extract::FromRef;
use axum::extract::MatchedPath;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use configuration::Settings;
pub use health_check::*;
use hyper::body::Bytes;
use hyper::{HeaderMap, Request};
pub use models::*;
use oauth2::basic::BasicClient;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::trace::TraceError;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_sdk::Resource;
use opentelemetry_stdout as stdout;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tower::BoxError;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing::{error, span};
use tracing::{info_span, Span};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

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

fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "youtinerary.api",
                "tracing-jaeger",
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tokio::main]
async fn main() -> Result<()> {
    let tracer = init_tracer()?;

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
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
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
                // .layer(OtelInResponseLayer::default())
                // .layer(OtelAxumLayer::default())
                .into_inner(),
        )
        // .route("", get(retrieve))
        .with_state(state);

    axum::serve(listener, router).await.unwrap();

    shutdown_tracer_provider();
    Ok(())
}
