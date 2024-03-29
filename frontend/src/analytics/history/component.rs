use yew::{classes, Context};

use crate::{
    common::FormattedPriceData,
    models::{Crypto, Price},
};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub label: String,
    pub price_change: Price,
    pub current_price: Price,
    pub use_absolute: bool,
    pub definition: Crypto,
}

pub struct Component {}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let properties = _ctx.props();
        let price_change = &properties.price_change;
        let current_price = &properties.current_price;

        let formatted_price_eur = FormattedPriceData::format_price(
            current_price.eur,
            price_change.eur,
            properties.use_absolute,
            properties.definition.precision,
        );
        let formatted_price_btc = FormattedPriceData::format_price(
            current_price.btc,
            price_change.btc,
            properties.use_absolute,
            6,
        );
        let formatted_price_eth = FormattedPriceData::format_price(
            current_price.eth,
            price_change.eth,
            properties.use_absolute,
            6,
        );

        // construct HTML
        yew::html! {
            <div class="history-column">
                <div class="label">{&properties.label}</div>
                <div class="prices-container">
                    <div class="prices">
                        <div class={classes!(&formatted_price_eur.change_direction, "price")}>
                            <div class="price-value">
                                {&formatted_price_eur.value}
                            </div>
                            <div class="price-change">
                                {&formatted_price_eur.change}
                            </div>
                        </div>
                        <div class={classes!(&formatted_price_btc.change_direction, "price")}>
                            <div class="price-value">
                                {&formatted_price_btc.value}
                            </div>
                            <div class="price-change">
                                {&formatted_price_btc.change}
                            </div>
                        </div>
                        <div class={classes!(&formatted_price_eth.change_direction, "price")}>
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

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}
