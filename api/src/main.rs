mod configuration;
mod database;
mod health_check;
mod logging;

use std::net::SocketAddr;

use anyhow::Context;
use anyhow::Result;
use api_core::AppState;
use api_features::itineraries_router;
use auth::authorize;
use auth::login_authorized;
use auth::protected;
use axum::routing::get;
use axum::Router;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use axum_tracing_opentelemetry::middleware::OtelInResponseLayer;
use configuration::Settings;
use database::connect_database;
use health_check::*;
use logging::init_tracer;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

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

    let state = AppState::new(
        pool,
        redis,
        settings.auth_settings.try_into()?,
        reqwest::Client::new(),
    );

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
