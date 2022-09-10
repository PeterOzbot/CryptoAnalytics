use std::rc::Rc;

use yew::{classes, Context};
use yewdux::prelude::Dispatch;

use crate::{
    common::FormattedPriceData,
    models::Crypto,
    store::{CryptoState, CryptoStore},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}

pub struct Component {
    _dispatch: Dispatch<CryptoStore>,
    state: Option<Rc<CryptoState>>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(Message::State));

        Self {
            _dispatch: dispatch,
            state: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let properties = _ctx.props();
        let mut html = yew::html! {
            <div class="general-row">
            <div class="loading-info">
                <div class="stage">
                    <div class="dot-carousel"></div>
                </div>
            </div>
        </div>
        };

        if let Some(state) = &self.state {
            if state
                .crypto_prices
                .contains_key(&properties.definition.api_key)
            {
                let prices_data = state.crypto_prices.get(&properties.definition.api_key);
                if let Some(data) = prices_data {
                    let formatted_price =
                        FormattedPriceData::format_data(data, properties.definition.precision);

                    // construct HTML
                    html = yew::html! {
                        <div class="general-row">
                            <div class="general-info">
                                <img alt={properties.definition.api_key.clone()} src={data.image.thumb.clone()}/>
                                <div class="general-price">
                                    <div class={classes!(&formatted_price.change_direction, "price")}>{formatted_price.value}</div>
                                    <div class={classes!(&formatted_price.change_direction, "change")}>{formatted_price.change}</div>
                                </div>
                            </div>

                            <crate::analytics::history::Component label="Now" price_change={data.market_data.price_change_percentage_24h_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=true />

                            <crate::analytics::history::Component label="24h" price_change={data.market_data.price_change_percentage_24h_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false />

                            <crate::analytics::history::Component label="7d" price_change={data.market_data.price_change_percentage_7d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                            <crate::analytics::history::Component label="30d" price_change={data.market_data.price_change_percentage_30d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                            <crate::analytics::history::Component label="200d" price_change={data.market_data.price_change_percentage_200d_in_currency.clone()}current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                            <crate::analytics::history::Component label="1y" price_change={data.market_data.price_change_percentage_1y_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                            <div class="legend">
                                <div>{"EUR"}</div>
                                <div>{"BTC"}</div>
                                <div>{"ETH"}</div>
                            </div>
                        </div>
                    }
                }
            }
        }
        html
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::State(state) => {
                self.state = Some(state);
            }
        }
        true
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
