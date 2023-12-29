#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct GasPriceData {
    pub message: String,
    pub result: GasPrice,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct GasPrice {
    #[serde(rename = "SafeGasPrice")]
    pub safe_gas_price: String,
    #[serde(rename = "ProposeGasPrice")]
    pub propose_gas_price: String,
    #[serde(rename = "FastGasPrice")]
    pub fast_gas_price: String,
}
