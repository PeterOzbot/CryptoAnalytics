use crate::{common::Error, models::PricesData};

pub enum Message {
    LoadPrices,
    PricesLoaded(Result<PricesData, Error>),
}
