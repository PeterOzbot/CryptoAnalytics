use crate::{
    common::Error,
    models::{Crypto, PricesData},
};

pub enum Message {
    Reload,
    DefinitionsLoaded(Result<Vec<Crypto>, Error>),
    PricesLoaded(String, Result<PricesData, Error>),
}
