use chrono::{self, Duration};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::Properties;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

use crate::common::price_formatting::PriceFormatting;
use crate::models::crypto::Crypto;
use crate::models::history_price::{self};
use crate::models::price::Price;

#[derive(Clone, PartialEq, Debug)]
pub enum HistoryDuration {
    Month,
    Year,
}

pub enum HistoryPriceMode {
    Btc,
    Eth,
}

#[derive(Debug)]
pub enum HistoryPriceOption {
    Loading,
    Some(history_price::HistoryCryptoData),
    Error,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CryptoHistoryProperties {
    pub duration: HistoryDuration,
    pub definition: Crypto,
    pub price: Price,
}

impl CryptoHistory {
    pub fn history_price_date(&self, duration: &HistoryDuration) -> String {
        let days = match duration {
            HistoryDuration::Month => -31,
            HistoryDuration::Year => -365,
        };
        let now = chrono::offset::Utc::now().date();
        let history_price_date = now + Duration::days(days);

        history_price_date.format("%d-%m-%Y").to_string()
    }

    pub fn handle_history_price(
        &self,
        current_price: f64,
        mode: HistoryPriceMode,
    ) -> (String, String, String) {
        match &self.history_price {
            HistoryPriceOption::Some(history) => {
                let history_price = match mode {
                    HistoryPriceMode::Btc => history.market_data.current_price.btc,
                    HistoryPriceMode::Eth => history.market_data.current_price.eth,
                };

                let price_diff = current_price - history_price;
                let price_change = (price_diff / current_price) * 100.0;
                (
                    PriceFormatting::format_price(history_price, 6),
                    format!("({:.2}%)", price_change),
                    PriceFormatting::handle_price_change(price_diff),
                )
            }
            HistoryPriceOption::Loading => (
                String::from("loading..."),
                String::from(""),
                String::from(""),
            ),
            HistoryPriceOption::Error => {
                (String::from("--------"), String::from("(-- %)"), String::from(""))
            }
        }
    }
}

pub struct CryptoHistory {
    properties: CryptoHistoryProperties,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    history_price: HistoryPriceOption,
}

impl Component for CryptoHistory {
    type Properties = CryptoHistoryProperties;
    type Message = history_price::Msg;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(history_price::Msg::MakeReq);
        Self {
            link,
            properties,
            fetch_task: None,
            history_price: HistoryPriceOption::Loading,
        }
    }

    fn view(&self) -> Html {
        // title
        let title = match self.properties.duration {
            HistoryDuration::Month => String::from("month:"),
            HistoryDuration::Year => String::from("year:"),
        };

        // against BTC/ETH historical
        let (
            formatted_history_price_btc,
            formatted_history_change_btc,
            formatted_history_change_direction_btc,
        ) = self.handle_history_price(self.properties.price.btc, HistoryPriceMode::Btc);
        let (
            formatted_history_price_eth,
            formatted_history_change_eth,
            formatted_history_change_direction_eth,
        ) = self.handle_history_price(self.properties.price.eth, HistoryPriceMode::Eth);

        html! {
            <div class="against-other">
                <div class="against-other-title align_center">{title}</div>
                <div class="against-other-data">
                    <div class=classes!(&formatted_history_change_direction_btc, "against-other-value")>{&formatted_history_price_btc}</div>
                    <div class=classes!(&formatted_history_change_direction_btc, "against-other-change")>{&formatted_history_change_btc}</div>
                </div>
                <div class="against-other-data">
                    <div class=classes!(&formatted_history_change_direction_eth, "against-other-value")>{&formatted_history_price_eth}</div>
                    <div class=classes!(&formatted_history_change_direction_eth, "against-other-change")>{&formatted_history_change_eth}</div>
                </div>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            history_price::Msg::MakeReq => {
                self.history_price = HistoryPriceOption::Loading;

                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}/history?date={:}&localization=false",self.properties.definition.id,self.history_price_date(&self.properties.duration) );
                ConsoleService::info(&format!("Loading history price data: {:?}", url_request));

                // create request
                let req = Request::get(url_request)
                    .body(Nothing)
                    .expect("Loading data failed");

                // callback to handle messaging
                let cb = self.link.callback(
                    |response: Response<
                        Json<Result<history_price::HistoryCryptoData, anyhow::Error>>,
                    >| {
                        let Json(data) = response.into_body();
                        history_price::Msg::Resp(data)
                    },
                );

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
            }
            history_price::Msg::Resp(resp) => match resp {
                Ok(data) => {
                    self.history_price = HistoryPriceOption::Some(data);
                    ConsoleService::info(&format!(
                        "History Price ({:?}): {:?}",
                        self.properties.duration, self.history_price
                    ));
                }
                Err(error) => {
                    self.history_price = HistoryPriceOption::Error;
                    ConsoleService::info(&format!(
                        "History Price ({:?}): {:?}",
                        self.properties.duration, error
                    ));
                }
            },
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.properties = properties;
        true
    }
}
