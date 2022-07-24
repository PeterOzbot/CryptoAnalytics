use serde::Serialize;
use sqlx::types::BigDecimal;

use super::Entry;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub definition_id: String,
    pub entries: Vec<Entry>,
    pub amount_sum: BigDecimal,
    pub buy_price_sum: BigDecimal,
    pub current_price_sum: BigDecimal,
}
