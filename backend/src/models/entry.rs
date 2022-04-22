use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct Entry {
    pub id: Uuid,
    pub definition_id: String,
    pub date_time: NaiveDateTime,
    pub amount: BigDecimal,
    pub withdraw_fee: BigDecimal,
    pub price: BigDecimal,
    pub purchase_fee: BigDecimal,
}

#[derive(Serialize, Debug)]
pub struct EntryResponse {
    pub id: Uuid,
    pub definition_id: String,
    pub date_time: NaiveDateTime,
    pub amount: String,
    pub withdraw_fee: String,
    pub price: String,
    pub purchase_fee: String,
}
