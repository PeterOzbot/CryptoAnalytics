#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct ApiData {
    pub image: Image,
    pub market_data: MarketData,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Image {
    pub thumb: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct MarketData {
    pub current_price: Price,
    pub price_change_24h_in_currency: Price,
    pub price_change_percentage_24h_in_currency: Price,
    pub price_change_percentage_7d_in_currency: Price,
    pub price_change_percentage_30d_in_currency: Price,
    pub price_change_percentage_200d_in_currency: Price,
    pub price_change_percentage_1y_in_currency: Price,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Price {
    pub eur: f64,
    pub btc: f64,
    pub eth: f64,
}
