use bigdecimal::{BigDecimal, ToPrimitive};

use super::PriceFormatting;

pub struct FormattedPortfolio {
    pub purchase_value: String,
    pub current_value: String,
    pub change: String,
    pub change_direction: String,
}

impl FormattedPortfolio {
    pub fn formatted_portfolio(
        purchase_price: &BigDecimal,
        current_price: &BigDecimal,
    ) -> FormattedPortfolio {
        // convert to float
        let purchase_float = FormattedPortfolio::get_float(purchase_price);
        let current_float = FormattedPortfolio::get_float(current_price);

        // calculate difference and % change
        let diff = current_float - purchase_float;
        let change = (diff / purchase_float) * 100f64;

        // format
        let formatted_difference_value = PriceFormatting::format_price(diff, 2);

        FormattedPortfolio {
            purchase_value: PriceFormatting::format_price(purchase_float, 2),
            current_value: PriceFormatting::format_price(current_float, 2),
            change_direction: PriceFormatting::handle_price_change(change),
            change: format!("({:}\u{00a0} {:.2}%)", formatted_difference_value, change),
        }
    }

    fn get_float(bigdecimal: &BigDecimal) -> f64 {
        if let Some(float_value) = bigdecimal.to_f64() {
            return float_value;
        }
        0f64
    }
}
