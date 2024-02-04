mod configuration;
mod error_handling;
mod features;
mod health_check;
mod middlewares;
mod models;

use std::net::SocketAddr;

use anyhow::Context;
use anyhow::Result;
use auth::authorize;
use auth::login_authorized;
use auth::protected;
use axum::extract::FromRef;
use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use configuration::Settings;
use features::itineraries_router;
use health_check::*;
use models::*;
use oauth2::basic::BasicClient;
use opentelemetry::trace::TraceError;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::Resource;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

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
                .with_endpoint("https://localhost:64849"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "youtinerary.api"
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
                .layer(OtelInResponseLayer)
                .layer(OtelAxumLayer::default())
                .into_inner(),
        )
        // .route("", get(retrieve))
        .with_state(state);

    axum::serve(listener, router).await.unwrap();

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
