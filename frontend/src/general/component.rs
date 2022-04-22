use yew::{
    classes,
    format::{Json, Nothing},
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

use crate::{
    common::FormattedPrice,
    models::{ApiData, Crypto},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}

pub struct Component {
    properties: Properties,
    data: Option<ApiData>,
    link: yew::ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(properties: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            properties,
            data: None,
            link,
            fetch_task: None,
        }
    }

    fn view(&self) -> yew::Html {
        if let Some(data) = &self.data {
            let formatted_price =
                FormattedPrice::format_data(data, self.properties.definition.precision);

            // construct HTML
            yew::html! {
                <div class="general-row">
                    <div class="general-info">
                        <img src=data.image.thumb.clone()/>
                        <div class="general-price">
                            <div class=classes!(&formatted_price.change_direction, "price")>{formatted_price.value}</div>
                            <div class=classes!(&formatted_price.change_direction, "change")>{formatted_price.change}</div>
                        </div>
                    </div>

                    <crate::history::Component label="24h" price_change=data.market_data.price_change_percentage_24h_in_currency.clone() current_price=data.market_data.current_price.clone() definition= self.properties.definition.clone() use_absolute=true />

                    <crate::history::Component label="7d" price_change=data.market_data.price_change_percentage_7d_in_currency.clone() current_price=data.market_data.current_price.clone() definition= self.properties.definition.clone() use_absolute=false/>

                    <crate::history::Component label="30d" price_change=data.market_data.price_change_percentage_30d_in_currency.clone() current_price=data.market_data.current_price.clone() definition= self.properties.definition.clone() use_absolute=false/>

                    <crate::history::Component label="200d" price_change=data.market_data.price_change_percentage_200d_in_currency.clone() current_price=data.market_data.current_price.clone() definition= self.properties.definition.clone() use_absolute=false/>

                    <crate::history::Component label="1y" price_change=data.market_data.price_change_percentage_1y_in_currency.clone() current_price=data.market_data.current_price.clone() definition= self.properties.definition.clone() use_absolute=false/>

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
    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::MakeReq => {
                self.data = None;

                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",self.properties.definition.api_key);
                ConsoleService::info(&format!(
                    "{:} -> Loading data: {:?}",
                    self.properties.definition.api_key, url_request
                ));

                // create request
                let req = Request::get(&url_request).body(Nothing).expect(
                    format!(
                        "Loading general data for {:} failed.",
                        self.properties.definition.api_key
                    )
                    .as_str(),
                );

                // callback to handle messaging
                let cb = self.link.callback(
                    |response: Response<Json<Result<ApiData, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Message::Resp(data)
                    },
                );

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect(&format!(
                    "{:} -> Fetch failed: {:?}",
                    self.properties.definition.api_key, url_request
                ));
                self.fetch_task = Some(task);
            }
            Message::Resp(resp) => match resp {
                Ok(data) => {
                    self.data = Some(data);
                    ConsoleService::info(&format!(
                        "{:} -> Loaded data: {:?}",
                        self.properties.definition.api_key, self.data
                    ));
                }
                Err(error) => {
                    ConsoleService::info(&format!(
                        "{:} -> Message response error: {:}",
                        self.properties.definition.api_key, error
                    ));
                }
            },
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        self.link.send_message(super::message::Message::MakeReq);
        false
    }
}
