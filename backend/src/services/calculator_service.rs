use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use bigdecimal::{BigDecimal, Zero};

use crate::models::{Entry, Portfolio};

pub fn calculate_portfolio(
    definition_id: &str,
    entries: Vec<Entry>,
    current_price: &str,
) -> Portfolio {
    let mut sum_amount: BigDecimal = BigDecimal::zero();
    let mut sum_price: BigDecimal = BigDecimal::zero();

    let mut portfolio_entries: Vec<Entry> = vec![];

    for entry in entries {
        sum_amount = sum_amount.add(&entry.amount);

        let amount = &entry.amount.clone().sub(&entry.withdraw_fee);
        let price = amount.mul(&entry.price);
        let final_price = price.sub(&entry.purchase_fee);
        sum_price = sum_price.add(final_price);

        portfolio_entries.push(entry);
    }

    let current_price_sum = calculate_current_price(current_price, &sum_amount);

    Portfolio {
        definition_id: String::from(definition_id),
        amount_sum: sum_amount,
        buy_price_sum: sum_price,
        current_price_sum: current_price_sum,
        entries: portfolio_entries,
    }
}

fn calculate_current_price(price: &str, sum_amount: &BigDecimal) -> BigDecimal {
    let current_price_raw = BigDecimal::from_str(price);
    if let Ok(current_price) = current_price_raw {
        return current_price.mul(sum_amount);
    }

    BigDecimal::from(0)
}
