use thousands::{SeparatorPolicy, digits, Separable};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Price {
    pub eur: f64,
    pub eur_24h_change: f64,
}

impl Price {
    pub fn handle_price_change(&self) -> Option<&'static str> {
        Some(match self.eur_24h_change {
            change if change < 0.0 => "red",
            change if change > 0.0 => "green",
            _ => "green",
        })
    }

    pub fn format_price(&self) -> String {
        let policy = SeparatorPolicy {
            separator: ",",
            groups:    &[3, 2],
            digits:    digits::ASCII_DECIMAL,
        };
        format!("{:} €", self.eur.separate_by_policy(policy))


        /*if let Some(price) = Decimal::from_f64(self.eur) {
            format!("{:}", Money::from_decimal(price, iso::EUR))
        }
        else{
            panic!("cant convert price")
        }*/
    }
}
