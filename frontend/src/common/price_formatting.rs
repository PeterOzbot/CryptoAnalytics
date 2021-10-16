use thousands::{Separable, SeparatorPolicy, digits};

pub struct PriceFormatting;

impl PriceFormatting {
    pub fn format_price(price: f64, precision: usize) -> String {
        let policy = SeparatorPolicy {
            separator: ",",
            groups: &[3, 2],
            digits: digits::ASCII_DECIMAL,
        };

        format!(
            "{:}",
            format!("{:.precision$}", price, precision = precision).separate_by_policy(policy)
        )
    }

    pub fn handle_price_change(price_change: f64) -> String {
        match price_change {
            change if change < 0.0 => String::from("red"),
            _ => String::from("green"),
        }
    }
}
