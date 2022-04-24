use crate::models::ApiData;

pub enum Message {
    LoadPrices,
    PricesLoaded(Result<ApiData, anyhow::Error>),
}
