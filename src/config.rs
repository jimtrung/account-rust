use std::env;

use dotenv::dotenv;
use sqlx::PgPool;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let db_url = env::var("DB_URL")
        .expect("Can not get DB_URL in .env");

    PgPool::connect(&db_url).await
}
