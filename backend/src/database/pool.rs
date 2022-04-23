use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DbPool {}

impl DbPool {
    pub async fn make_pool() -> PgPool {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            if let Ok(pool) = PgPoolOptions::new().connect(&db_url).await {
                pool
            } else {
                panic!("Connection to the database: {:} failed.", &db_url)
            }
        } else {
            panic!("DATABASE_URL missing in .env.");
        }
    }
}
