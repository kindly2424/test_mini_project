use sqlx::mysql::MySqlPoolOptions;
use std::env;

pub async fn connect_db() -> sqlx::MySqlPool {
    dotenvy::dotenv().ok();

    let user = env::var("DB_USER").unwrap();
    let pass = env::var("DB_PASS").unwrap_or_default();
    let host = env::var("DB_HOST").unwrap();
    let port = env::var("DB_PORT").unwrap();
    let name = env::var("DB_NAME").unwrap();

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, pass, host, port, name
    );

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Gagal connect database MySQL")
}