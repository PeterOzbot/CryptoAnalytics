use super::{Data, PriceFormatting};

pub struct FormattedPrice {
    pub value: String,
    pub change_direction: String,
    pub change: String,
}

impl FormattedPrice {
    pub fn format_data(data: &Data, precision: usize) -> FormattedPrice {
        FormattedPrice {
            value: PriceFormatting::format_price(data.market_data.current_price.eur, precision),
            change_direction: PriceFormatting::handle_price_change(
                data.market_data.price_change_24h_in_currency.eur,
            ),
            change: format!(
                "({:.2}% \u{00a0} {:})",
                data.market_data.price_change_percentage_24h_in_currency.eur,
                PriceFormatting::format_price(
                    data.market_data.price_change_24h_in_currency.eur,
                    precision,
                )
            ),
        }
    }

    pub fn format_price(
        current_price: f64,
        price_change: f64,
        use_absolute: bool,
        precision: usize,
    ) -> FormattedPrice {
        FormattedPrice {
            value: FormattedPrice::get_price(current_price, price_change, use_absolute, precision),
            change_direction: PriceFormatting::handle_price_change(price_change),
            change: format!("({:.2}%)", price_change,),
        }
    }

    fn get_price(
        current_price: f64,
        price_change: f64,
        use_absolute: bool,
        precision: usize,
    ) -> String {
        if use_absolute {
            PriceFormatting::format_price(current_price, precision)
        } else {
            PriceFormatting::format_price(
                current_price / (1f64 + (price_change / 100f64)),
                precision,
            )
        }
    }
}
