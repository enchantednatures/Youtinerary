use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub(crate) async fn connect_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}
