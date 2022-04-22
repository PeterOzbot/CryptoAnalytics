use sqlx::{query_as, PgPool};

use crate::models::{Definition, Entry};

pub struct Provider {}

impl Provider {
    pub async fn get_definitions(db_pool: &PgPool) -> Result<Vec<Definition>, sqlx::Error> {
        query_as!(
            Definition,
            r#"
            SELECT api_key, precision from definitions
            "#
        )
        .fetch_all(db_pool)
        .await
    }

    pub async fn get_entries(
        definition_id: &str,
        db_pool: &PgPool,
    ) -> Result<Vec<Entry>, sqlx::Error> {
        query_as!(
            Entry,
            r#"
            SELECT id, definition_id, date_time, amount, withdraw_fee, price, purchase_fee from entries
            WHERE definition_id = $1
            "#,
            definition_id
        )
        .fetch_all(db_pool)
        .await
    }
}
