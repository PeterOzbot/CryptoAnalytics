use serde::Serialize;
use sqlx::types::BigDecimal;

use super::Entry;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub definition_id: String,
    pub bought_entries: Vec<Entry>,
    pub sold_entries: Vec<Entry>,
    pub crypto_amount_sum: BigDecimal,
    pub bought_fiat_price_sum: BigDecimal,
    pub fiat_current_total_price: BigDecimal,
    pub bought_fiat_average_price: BigDecimal,
    pub sold_fiat_average_price: BigDecimal,
    pub sold_fiat_price_sum: BigDecimal,
}
