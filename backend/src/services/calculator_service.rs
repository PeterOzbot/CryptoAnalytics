use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use bigdecimal::BigDecimal;

use crate::models::{Entry, Portfolio, TransactionType};

pub fn calculate_portfolio(
    definition_id: &str,
    entries: &[Entry],
    current_price: &str,
) -> Portfolio {
    // sold entries
    let sold_entries = get_sell_entries(entries);

    let (sold_crypto_sum_amount, sold_fiat_sum_price, _, sold_portfolio_entries) =
        process_entries(sold_entries);

    // bought entries
    let bought_entries = get_buy_entries(entries);

    let (
        bought_crypto_sum_amount,
        bought_fiat_sum_price,
        bought_fiat_average_price,
        bought_portfolio_entries,
    ) = process_entries(bought_entries);

    // calculate current total price
    let fiat_current_total_price = calculate_fiat_current_total_price(
        current_price,
        &bought_crypto_sum_amount,
        &sold_crypto_sum_amount,
    );

    Portfolio {
        definition_id: String::from(definition_id),
        crypto_amount_sum: bought_crypto_sum_amount.sub(sold_crypto_sum_amount.clone()),
        bought_fiat_price_sum: bought_fiat_sum_price,
        fiat_current_total_price: fiat_current_total_price,
        bought_entries: bought_portfolio_entries,
        sold_entries: sold_portfolio_entries,
        bought_fiat_average_price: bought_fiat_average_price.clone(),
        sold_fiat_average_price: sold_crypto_sum_amount.mul(bought_fiat_average_price),
        sold_fiat_price_sum: sold_fiat_sum_price,
    }
}

fn process_entries(entries: Vec<&Entry>) -> (BigDecimal, BigDecimal, BigDecimal, Vec<Entry>) {
    let entries_len = entries.len() as u64;

    let mut crypto_sum_amount = BigDecimal::from(0);
    let mut fiat_sum_price = BigDecimal::from(0);
    let mut fiat_price_sum = BigDecimal::from(0);
    let mut portfolio_entries = Vec::new();

    for entry in entries {
        crypto_sum_amount = crypto_sum_amount.add(&entry.amount);

        let crypto_amount = &entry.amount.clone().sub(&entry.withdraw_fee);
        let fiat_price_net = crypto_amount.mul(&entry.price);
        let fiat_price_gross = fiat_price_net.sub(&entry.transaction_fee);

        fiat_sum_price = fiat_sum_price.add(fiat_price_gross);
        fiat_price_sum = fiat_price_sum.add(&entry.price);

        portfolio_entries.push((*entry).clone());
    }

    // calculate average price
    let fiat_average_price = match entries_len {
        0 => BigDecimal::from(0),
        _ => fiat_price_sum.div(BigDecimal::from(entries_len)),
    };

    return (
        crypto_sum_amount,
        fiat_sum_price,
        fiat_average_price,
        portfolio_entries,
    );
}

fn calculate_fiat_current_total_price(
    current_price_raw: &str,
    bought_sum_crypto_amount: &BigDecimal,
    sold_sum_crypto_amount: &BigDecimal,
) -> BigDecimal {
    let current_price_parsed = BigDecimal::from_str(current_price_raw);
    if let Ok(current_price) = current_price_parsed {
        return current_price.mul(bought_sum_crypto_amount.sub(sold_sum_crypto_amount));
    }

    BigDecimal::from(0)
}

pub fn get_buy_entries<'a>(entries: &'a [Entry]) -> Vec<&'a Entry> {
    entries
        .iter()
        .filter(|entry| entry.transaction_type == TransactionType::Bought)
        .collect()
}

pub fn get_sell_entries<'a>(entries: &'a [Entry]) -> Vec<&'a Entry> {
    entries
        .iter()
        .filter(|entry| entry.transaction_type == TransactionType::Sold)
        .collect()
}
