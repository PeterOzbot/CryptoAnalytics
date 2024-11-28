use bigdecimal::BigDecimal;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Entry {
    pub definition_id: String,       // crypto api_key
    pub date_time: String,           // when was purchased
    pub amount: BigDecimal,          // how much of crypto currency
    pub withdraw_fee: BigDecimal,    // fee to withdraw from exchange - in crypto
    pub price: BigDecimal,           // price in EUR
    pub transaction_fee: BigDecimal, // exchange fee in EUR
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Portfolio {
    pub definition_id: String,
    pub bought_entries: Vec<Entry>,
    pub sold_entries: Vec<Entry>,
    pub crypto_amount_sum: BigDecimal,
    pub bought_fiat_price_sum: BigDecimal,
    pub fiat_current_total_price: BigDecimal,
    pub bought_fiat_average_price: BigDecimal,
    pub sold_fiat_average_price: BigDecimal,
    pub sold_fiat_price_sum: BigDecimal,
}
