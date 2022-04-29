use crate::models::PricesData;

pub enum Message {
    LoadPrices,
    PricesLoaded(Result<PricesData, anyhow::Error>),
}
