use bigdecimal::{BigDecimal, ToPrimitive};

use std::ops::Sub;

use super::PriceFormatting;

pub struct FormattedPortfolio {
    pub purchase_value: String,
    pub current_value: String,
    pub change: String,
    pub change_direction: String,

    pub sold_value: String,
    pub sold_change: String,
    pub sold_change_direction: String,
}

impl FormattedPortfolio {
    pub fn formatted_portfolio(
        purchase_price: &BigDecimal,
        current_price: &BigDecimal,
        sold_amount: &BigDecimal,
        sold_price_sum: &BigDecimal,
    ) -> FormattedPortfolio {
        // convert to float
        let purchase_float = FormattedPortfolio::get_float(&purchase_price.sub(sold_amount));
        let current_float = FormattedPortfolio::get_float(current_price);
        let sold_float = FormattedPortfolio::get_float(sold_amount);
        let sold_price_sum_float = FormattedPortfolio::get_float(sold_price_sum);

        // calculate difference and % change
        let diff = current_float - purchase_float;
        let change = (diff / purchase_float) * 100f64;

        let sold_diff = sold_float - sold_price_sum_float;
        let sold_change = (sold_diff / sold_price_sum_float) * 100f64;

        // format
        let formatted_difference_value = PriceFormatting::format_price(diff, 2);
        let formatted_difference_sold_value = PriceFormatting::format_price(sold_diff, 2);

        FormattedPortfolio {
            purchase_value: PriceFormatting::format_price(purchase_float, 2),
            current_value: PriceFormatting::format_price(current_float, 2),
            change_direction: PriceFormatting::handle_price_change(change),
            change: format!("({:}\u{00a0} {:.2}%)", formatted_difference_value, change),

            sold_value: PriceFormatting::format_price(sold_float, 2),
            sold_change: format!(
                "({:}\u{00a0} {:.2}%)",
                formatted_difference_sold_value, sold_change
            ),
            sold_change_direction: PriceFormatting::handle_price_change(sold_change),
        }
    }

    fn get_float(bigdecimal: &BigDecimal) -> f64 {
        if let Some(float_value) = bigdecimal.to_f64() {
            return float_value;
        }
        0f64
    }
}
