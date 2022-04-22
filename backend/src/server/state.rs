use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: PgPool,
}
