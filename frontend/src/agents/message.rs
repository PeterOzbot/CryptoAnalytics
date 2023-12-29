use crate::{
    common::Error,
    models::{Crypto, GasPriceData, Portfolio, PricesData},
};

pub enum Message {
    Reload,
    PortfolioLoaded(String, Result<Portfolio, Error>),
    DefinitionsLoaded(Result<Vec<Crypto>, Error>),
    PricesLoaded(String, Result<PricesData, Error>),
    GasPriceLoaded(Result<GasPriceData, Error>),
}
