use gloo_console::info;
use yew::{classes, Context};

use crate::{
    common::{request_get, FormattedPrice},
    models::{Crypto, PricesData},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}

pub struct Component {
    data: Option<PricesData>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(super::message::Message::LoadPrices);
        Self { data: None }
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let properties = _ctx.props();
        if let Some(data) = &self.data {
            let formatted_price =
                FormattedPrice::format_data(data, properties.definition.precision);

            // construct HTML
            yew::html! {
                <div class="general-row">
                    <div class="general-info">
                        <img alt={properties.definition.api_key.clone()} src={data.image.thumb.clone()}/>
                        <div class="general-price">
                            <div class={classes!(&formatted_price.change_direction, "price")}>{formatted_price.value}</div>
                            <div class={classes!(&formatted_price.change_direction, "change")}>{formatted_price.change}</div>
                        </div>
                    </div>

                    <crate::analytics::history::Component label="24h" price_change={data.market_data.price_change_percentage_24h_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=true />

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
        } else {
            // loading indicator
            yew::html! {
                <div class="general-row">
                <div class="loading-info">
                    <div class="stage">
                        <div class="dot-carousel"></div>
                    </div>
                </div>
            </div>
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let properties = ctx.props().clone();
        match msg {
            Message::LoadPrices => {
                self.data = None;

                ctx.link().send_future(async move {
                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",properties.definition.api_key);
                info!(&format!(
                    "General component: {:} -> Loading data: {:?}",
                    properties.definition.api_key, url_request
                ));

                let response = request_get::<PricesData>(url_request).await;

                Message::PricesLoaded(response)
                });
            }
            Message::PricesLoaded(resp) => match resp {
                Ok(data) => {
                    self.data = Some(data);
                    info!(&format!(
                        "General component: {:} -> Loaded data: {:?}",
                        properties.definition.api_key, self.data
                    ));
                }
                Err(error) => {
                    info!(&format!(
                        "General component: {:} -> Response error: {:}",
                        properties.definition.api_key, error
                    ));
                }
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(super::message::Message::LoadPrices);
        false
    }
}
