use sqlx::{
    mysql::MySqlPoolOptions,
    MySqlPool,
};

pub async fn connect_db() -> MySqlPool {

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");

    MySqlPoolOptions::new()

        .max_connections(5)

        .connect(&database_url)

        .await

        .expect("Cannot connect DB")
}