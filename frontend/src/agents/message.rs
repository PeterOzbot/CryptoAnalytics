use crate::{
    common::Error,
    models::{Crypto, Entry, PricesData},
};

pub enum Message {
    Reload,
    PortfolioLoaded(String, Result<Vec<Entry>, Error>),
    DefinitionsLoaded(Result<Vec<Crypto>, Error>),
    PricesLoaded(String, Result<PricesData, Error>),
}
