use sqlx::{PgPool, Pool};

pub struct DbPool {}

impl DbPool {
    pub async fn make_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        Pool::new(&db_url).await.unwrap()
    }
}
