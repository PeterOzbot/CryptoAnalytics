use yew::classes;

use crate::common::{data::Price, price_formatting::PriceFormatting};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub label: String,
    pub price_change: Price,
    pub current_price: Price,
}

pub struct Component {
    properties: Properties,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(properties: Self::Properties, _: yew::ComponentLink<Self>) -> Self {
        Self { properties }
    }

    fn view(&self) -> yew::Html {
        let price_change = &self.properties.price_change;
        let label = &self.properties.label;
        let current_price = &self.properties.current_price;

        let change_direction1 = PriceFormatting::handle_price_change(price_change.eur);
        let change_direction2 = PriceFormatting::handle_price_change(price_change.btc);
        let change_direction3 = PriceFormatting::handle_price_change(price_change.eth);

        let history_price1 = current_price.eur / (1f64 + price_change.eur);
        let history_price2 = current_price.btc / (1f64 + price_change.btc);
        let history_price3 = current_price.eth / (1f64 + price_change.eth);

        // construct HTML
        yew::html! {
            <div class="history-column">
                <div class="label">{label}</div>
                <div class=classes!(change_direction1, "price")>{history_price1}</div>
                <div class=classes!(change_direction2, "price")>{history_price2}</div>
                <div class=classes!(change_direction3, "price")>{history_price3}</div>
            </div>
        }
    }
    fn update(&mut self, _: Self::Message) -> yew::ShouldRender {
        true
    }

    fn change(&mut self, properties: Self::Properties) -> yew::ShouldRender {
        self.properties = properties;
        true
    }
}
