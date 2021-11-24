use super::data::Data;

pub struct FormattedData {
    pub icon: String,
    pub price: FormattedPrice,
}

impl FormattedData {
    pub fn format(data: &Data) -> FormattedData {
        FormattedData {
            icon: data.image.thumb.clone(),
            price: FormattedPrice {
                eur: String::from(""),
                btc: String::from(""),
                eth: String::from(""),
            },
        }
    }
}
pub struct FormattedPrice {
    pub eur: String,
    pub btc: String,
    pub eth: String,
}
