use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: Uuid,
    pub definition_id: String,    // crypto api_key
    pub date_time: NaiveDateTime, // when was purchased
    pub amount: BigDecimal,       // how much of crypto currency
    pub withdraw_fee: BigDecimal, // fee to withdraw from exchange - in crypto
    pub price: BigDecimal,        // price in EUR
    pub purchase_fee: BigDecimal, // exchange fee in EUR
}
