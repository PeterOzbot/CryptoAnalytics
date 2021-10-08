use super::price::Price;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Cryptos {
    pub bitcoin: Price,
    pub ethereum: Price,
    pub litecoin: Price,
    #[serde(rename(deserialize = "bitcoin-cash"))]
    pub bitcoin_cash: Price,
    #[serde(rename(deserialize = "chainlink"))]
    pub chain_link: Price,
}
