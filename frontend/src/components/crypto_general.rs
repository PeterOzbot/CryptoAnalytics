use yew::Properties;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

use crate::common::price_formatting::PriceFormatting;
use crate::components::crypto_history::{CryptoHistory, HistoryDuration};
use crate::models::crypto::Crypto;
use crate::models::history_price::{self};
use crate::models::price;

#[derive(Properties, Clone, PartialEq)]
pub struct CryptoGeneralProperties {
    pub price: price::Price,
    pub definition: Crypto,
}

pub struct CryptoGeneral {
    properties: CryptoGeneralProperties
}

impl Component for CryptoGeneral {
    type Properties = CryptoGeneralProperties;
    type Message = history_price::Msg;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(history_price::Msg::MakeReq);
        Self {
            properties,
        }
    }

    fn view(&self) -> Html {
        let properties = self.properties.clone();
        // current price
        let price_change_direction =
            PriceFormatting::handle_price_change(properties.price.eur_24h_change);
        let formatted_price = PriceFormatting::format_price(properties.price.eur, 2);
        let formatted_change = format!("{:.2} %", properties.price.eur_24h_change);

        // against BTC/ETH
        let formatted_price_btc = PriceFormatting::format_price(properties.price.btc, 6);
        let formatted_change_btc = format!("({:.2}%) ", properties.price.btc_24h_change);
        let formatted_change_direction_btc =
            PriceFormatting::handle_price_change(properties.price.btc_24h_change);
        let formatted_price_eth = PriceFormatting::format_price(properties.price.eth, 6);
        let formatted_change_eth = format!("({:.2}%) ", properties.price.eth_24h_change);
        let formatted_change_direction_eth =
            PriceFormatting::handle_price_change(properties.price.eth_24h_change);

        // construct HTML
        html! {
            <div class="crypto-general-container">
                <div class="crypto-general-container-inner">
                    <div class="align_center crypto-general-main-price">
                        <img src={properties.definition.icon} class="image"/>
                        <span class=classes!(price_change_direction.clone(), "price")>{formatted_price}</span>
                        <span class=classes!(price_change_direction.clone(), "change")>{formatted_change}</span>
                    </div>

                    <div class="against-other-container align_center">
                        <div class="against-other">

                            <div class="against-other-title align_center">{"day:"}</div>
                            <div class="against-other-data">
                                <div class="against-other-value">{"BTC:"}</div>
                                <div class=classes!(&formatted_change_direction_btc, "against-other-value")>{&formatted_price_btc}</div>
                                <div class=classes!(&formatted_change_direction_btc, "against-other-change")>{&formatted_change_btc}</div>
                            </div>
                            <div class="against-other-data">
                                <div class="against-other-value">{"ETH:"}</div>
                                <div class=classes!(&formatted_change_direction_eth, "against-other-value")>{&formatted_price_eth}</div>
                                <div class=classes!(&formatted_change_direction_eth, "against-other-change")>{&formatted_change_eth}</div>
                            </div>
                        </div>

                     <CryptoHistory definition=self.properties.definition.clone() price=self.properties.price.clone() duration=HistoryDuration::Month/>
                     
                     <CryptoHistory definition=self.properties.definition.clone() price=self.properties.price.clone() duration=HistoryDuration::Year/>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.properties = properties;
        true
    }
}
