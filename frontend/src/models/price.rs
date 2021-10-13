use serde_json::Value;

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
    Resp(Result<Value, anyhow::Error>),
}