use chrono::{self, Duration};
use thousands::{digits, Separable, SeparatorPolicy};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::Properties;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

use crate::models::crypto::Crypto;
use crate::models::history_price::{self};
use crate::models::price;

#[derive(Properties, Clone, PartialEq)]
pub struct CryptoGeneralProperties {
    pub price: price::Price,
    pub definition: Crypto,
}

pub struct CryptoGeneral {
    properties: CryptoGeneralProperties,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    history_price: Option<history_price::HistoryCryptoData>,
}

impl CryptoGeneral {
    pub fn format_price(&self, price: f64, precision: usize) -> String {
        let policy = SeparatorPolicy {
            separator: ",",
            groups: &[3, 2],
            digits: digits::ASCII_DECIMAL,
        };

        format!(
            "{:}",
            format!("{:.precision$}", price, precision = precision).separate_by_policy(policy)
        )
    }

    pub fn handle_price_change(&self, price_change: f64) -> String {
        match price_change {
            change if change < 0.0 => String::from("red"),
            _ => String::from("green"),
        }
    }

    pub fn history_price_date(&self) -> String {
        let now = chrono::offset::Utc::now().date();
        let history_price_date = now + Duration::days(-31);

        history_price_date.format("%d-%m-%Y").to_string()
    }

    pub fn handle_history_price(&self, current_price: f64, mode: &str) -> (String, String, String) {
        if let Some(history) = &self.history_price {
            let history_price = match mode {
                "btc" => history.market_data.current_price.btc,
                "eth" | _ => history.market_data.current_price.eth,
            };

            let price_diff = current_price - history_price;
            let price_change = (price_diff / current_price) * 100.0;
            (
                self.format_price(price_diff, 6),
                format!("({:.2}%)", price_change),
                self.handle_price_change(price_diff),
            )
        } else {
            (
                String::from("loading..."),
                String::from(""),
                String::from(""),
            )
        }
    }
}
impl Component for CryptoGeneral {
    type Properties = CryptoGeneralProperties;
    type Message = history_price::Msg;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(history_price::Msg::MakeReq);
        Self {
            link,
            properties,
            fetch_task: None,
            history_price: None,
        }
    }

    fn view(&self) -> Html {
        let properties = self.properties.clone();
        // current price
        let price_change_direction = self.handle_price_change(properties.price.eur_24h_change);
        let formatted_price = self.format_price(properties.price.eur, 2);
        let formatted_change = format!("{:.2} %", properties.price.eur_24h_change);

        // against BTC/ETH
        let formatted_price_btc = self.format_price(properties.price.btc, 6);
        let formatted_change_btc = format!(" ({:.2}%) ", properties.price.btc_24h_change);
        let formatted_price_eth = self.format_price(properties.price.eth, 6);
        let formatted_change_eth = format!(" ({:.2}%) ", properties.price.eth_24h_change);

        // against BTC/ETH historical
        let (
            formatted_history_diff_btc,
            formatted_history_change_btc,
            formatted_history_change_direction_btc,
        ) = self.handle_history_price(self.properties.price.btc, "btc");
        let (
            formatted_history_diff_eth,
            formatted_history_change_eth,
            formatted_history_change_direction_eth,
        ) = self.handle_history_price(self.properties.price.eth, "eth");

        // construct HTML
        html! {
            <div class="crypto-general-container">
                <div class="crypto-general-container-inner">
                    <div class="current_price align_center">
                        <img src={properties.definition.icon} class="image"/>
                        <span class=classes!(price_change_direction.clone(), "price")>{formatted_price}</span>
                        <span class=classes!(price_change_direction.clone(), "change")>{formatted_change}</span>
                    </div>

                    <div class="against_btc align_center">
                        <span class=classes!("against_other")>{"BTC: "}{formatted_price_btc}{formatted_change_btc}</span>
                        <span class=classes!(formatted_history_change_direction_btc, "against_other")>{" m: "}{formatted_history_diff_btc}{formatted_history_change_btc}</span>
                    </div>

                    <div class="against_eth align_center">
                        <span class=classes!("against_other")>{"ETH: "}{formatted_price_eth}{formatted_change_eth}</span>
                        <span class=classes!(formatted_history_change_direction_eth, "against_other")>{" m: "}{formatted_history_diff_eth}{formatted_history_change_eth}</span>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            history_price::Msg::MakeReq => {
                self.history_price = None;

                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}/history?date={:}&localization=false",self.properties.definition.id,self.history_price_date() );
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
            history_price::Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.history_price = Some(data);
                    ConsoleService::info(&format!("History Price: {:?}", self.history_price));
                }
            }
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> ShouldRender {
        self.properties = properties;
        true
    }
}
