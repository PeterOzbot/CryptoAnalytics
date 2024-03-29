use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use chrono::Local;

use crate::models::{Crypto, GasPriceData, Portfolio, PricesData};

#[derive(Clone, Default)]
pub struct CryptoState {
    pub last_updated: Option<chrono::DateTime<Local>>,
    pub crypto_definitions: Option<Vec<Crypto>>,
    pub crypto_prices: HashMap<String, PricesData>,
    pub portfolio: HashMap<String, Portfolio>,
    pub gas_price: Option<GasPriceData>,
}

impl CryptoState {
    pub fn new() -> CryptoState {
        CryptoState {
            last_updated: Some(chrono::offset::Local::now()),
            crypto_definitions: None,
            crypto_prices: HashMap::new(),
            portfolio: HashMap::new(),
            gas_price: None,
        }
    }
}

impl Display for CryptoState {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let formatted_last_updated = match self.last_updated {
            Some(date) => date.format("%d.%m ~ %H:%M").to_string(),
            None => String::from("/ ~ /"),
        };

        fmt.write_str(format!("LastUpdated: {:}", formatted_last_updated).as_str())?;
        Ok(())
    }
}
