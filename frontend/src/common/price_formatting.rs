use thousands::{digits, Separable, SeparatorPolicy};

pub struct PriceFormatting;

impl PriceFormatting {
    pub fn format_price(price: f64, precision: i16) -> String {
        let policy = SeparatorPolicy {
            separator: ",",
            groups: &[3, 2],
            digits: digits::ASCII_DECIMAL,
        };

        format!(
            "{:}",
            format!(
                "{:.precision_value$}",
                price,
                precision_value = PriceFormatting::convert(precision)
            )
            .separate_by_policy(policy)
        )
    }

    pub fn handle_price_change(price_change: f64) -> String {
        match price_change {
            change if change < 0.0 => String::from("red"),
            _ => String::from("green"),
        }
    }

    fn convert(precision: i16) -> usize {
        let conversion = usize::try_from(precision);

        match conversion {
            Ok(value) => value,
            _ => 2,
        }
    }
}
