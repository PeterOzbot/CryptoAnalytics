use super::{data::Data, price_formatting::PriceFormatting};

pub struct FormattedData {
    pub icon: String,
    pub current: FormattedPrice,
    //pub history_eur_7d: FormattedPrice,
    // pub btc: FormattedPrice,
    // pub eth: FormattedPrice,
}

impl FormattedData {
    pub fn format(data: &Data) -> FormattedData {
        FormattedData {
            icon: data.image.thumb.clone(),
            current: FormattedPrice {
                value: PriceFormatting::format_price(data.market_data.current_price.eur, 2),
                change_direction: PriceFormatting::handle_price_change(
                    data.market_data.price_change_24h_in_currency.eur,
                ),
                change: format!(
                    "({:.2}% \u{00a0} {:})",
                    data.market_data.price_change_percentage_24h_in_currency.eur,
                    PriceFormatting::format_price(
                        data.market_data.price_change_24h_in_currency.eur,
                        2,
                    )
                ),
            },
        }
    }
}

// 3000÷(1+0,79)
// current / (1 + change)

pub struct FormattedPrice {
    pub value: String,
    pub change_direction: String,
    pub change: String,
}
