use std::fmt::{Display, Formatter, Result};

use super::price::Price;

#[derive(Clone, PartialEq, Debug)]
pub struct CryptoData {
    pub definition: Crypto,
    pub price: Price,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Crypto {
    pub id: &'static str,
    pub icon: &'static str,
}

impl Display for Crypto {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(self.id)?;
        Ok(())
    }
}
