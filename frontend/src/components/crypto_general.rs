use yew::Properties;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

use crate::models::price::Price;

#[derive(Properties, Clone, PartialEq)]
pub struct CryptoGeneralProperties {
    pub price: Price,
    pub name: String,
    pub image: String,
}

pub struct CryptoGeneral {
    properties: CryptoGeneralProperties,
}

pub enum Msg {}

impl Component for CryptoGeneral {
    type Properties = CryptoGeneralProperties;
    type Message = Msg;

    fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { properties }
    }

    fn view(&self) -> Html {
        let properties = self.properties.clone();
        let price_change_direction = properties.price.handle_price_change();
        let formatted_price = properties.price.format_price();
        let formatted_change = format!("{:.2} %", properties.price.eur_24h_change);

        html! {
            <div class="crypto-general-container">
                <img src={properties.image} class="image"/>
                // <span class="name">{properties.name.clone()}{" "}</span>
                <span class=classes!(price_change_direction, "price")>{formatted_price}</span>
                <span class=classes!(price_change_direction, "change")>{formatted_change}</span>
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.properties = properties;
        true
    }
}
