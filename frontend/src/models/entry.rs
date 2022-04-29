use bigdecimal::BigDecimal;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Entry {
    pub definition_id: String,    // crypto api_key
    pub date_time: String,        // when was purchased
    pub amount: BigDecimal,       // how much of crypto currency
    pub withdraw_fee: BigDecimal, // fee to withdraw from exchange - in crypto
    pub price: BigDecimal,        // price in EUR
    pub purchase_fee: BigDecimal, // exchange fee in EUR
}
