#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Cryptos {
    pub bitcoin: Price,
    pub ethereum: Price,
    pub litecoin: Price,
    #[serde(rename(deserialize = "bitcoin-cash"))]
    pub bitcoin_cash: Price,
    #[serde(rename(deserialize = "chainlink"))]
    pub chain_link: Price,
    #[serde(rename(deserialize = "unit-protocol-duck"))]
    pub unit_protocol_duck:Price,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Price {
    pub eur: f64,
    pub eur_24h_change: f64,
    pub eth: f64,
    pub eth_24h_change: f64,
    pub btc: f64,
    pub btc_24h_change: f64,
}

pub enum Msg {
    MakeReq,
    Resp(Result<Cryptos, anyhow::Error>),
}