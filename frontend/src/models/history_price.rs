#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct HistoryCryptoData {
    pub market_data: MarketData,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct MarketData {
    pub current_price: Price,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Price {
    pub eur: f64,
    pub btc: f64,
    pub eth: f64,
}

pub enum Msg {
    MakeReq,
    Resp(Result<HistoryCryptoData, anyhow::Error>),
}
