use yew::classes;

use crate::{
    common::{FormattedPrice, Price, PriceFormatting},
    models::crypto::Crypto,
};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub label: String,
    pub price_change: Price,
    pub current_price: Price,
    pub use_absolute: bool,
    pub definition: Crypto,
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
        let current_price = &self.properties.current_price;

        let formatted_price_eur = FormattedPrice::format_price(
            current_price.eur,
            price_change.eur,
            self.properties.use_absolute,
            self.properties.definition.precision,
        );
        let formatted_price_btc = FormattedPrice::format_price(
            current_price.btc,
            price_change.btc,
            self.properties.use_absolute,
            6,
        );
        let formatted_price_eth = FormattedPrice::format_price(
            current_price.eth,
            price_change.eth,
            self.properties.use_absolute,
            6,
        );

        // construct HTML
        yew::html! {
            <div class="history-column">
                <div class="label">{&self.properties.label}</div>
                <div class="prices-container">
                    <div class="prices">
                        <div class=classes!(&formatted_price_eur.change_direction, "price")>
                            <div class="price-value">
                                {&formatted_price_eur.value}
                            </div>
                            <div class="price-change">
                                {&formatted_price_eur.change}
                            </div>
                        </div>
                        <div class=classes!(&formatted_price_btc.change_direction, "price")>
                            <div class="price-value">
                                {&formatted_price_btc.value}
                            </div>
                            <div class="price-change">
                                {&formatted_price_btc.change}
                            </div>
                        </div>
                        <div class=classes!(&formatted_price_eth.change_direction, "price")>
                            <div class="price-value">
                                {&formatted_price_eth.value}
                            </div>
                            <div class="price-change">
                                {&formatted_price_eth.change}
                            </div>
                        </div>
                    </div>
                </div>
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
