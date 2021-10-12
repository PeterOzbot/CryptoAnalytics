#[macro_use]
extern crate serde;

use std::time::Duration;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::{
    classes,
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        timeout::TimeoutTask,
        ConsoleService, FetchService, TimeoutService,
    },
    App, Component, ComponentLink, Html, ShouldRender,
};

use components::crypto_general::CryptoGeneral;
use models::price::*;

mod components;
mod models;

struct CryptoAnalyticsApp {
    link: ComponentLink<Self>,
    cryptos: Option<Cryptos>,
    fetch_task: Option<FetchTask>,
    refresh_task: Option<TimeoutTask>,
}

impl Component for CryptoAnalyticsApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);

        Self {
            link,
            cryptos: None,
            fetch_task: None,
            refresh_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.cryptos = None;

                // url for request
                let url_request = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,litecoin,ethereum,bitcoin-cash,chainlink,unit-protocol-duck&vs_currencies=EUR,BTC,ETH&include_24hr_change=true";
                ConsoleService::info(&format!("Loading data: {:?}", url_request));

                // create request
                let req = Request::get(url_request)
                    .body(Nothing)
                    .expect("Loading data failed");

                // callback to handle messaging
                let cb = self.link.callback(
                    |response: Response<Json<Result<Cryptos, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);

                // set recurring calls
                self.refresh_task = Some(TimeoutService::spawn(
                    Duration::from_secs(300),
                    self.link.callback(|_| Msg::MakeReq),
                ));
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.cryptos = Some(data);
                    ConsoleService::info(&format!("Cryptos: {:?}", self.cryptos));
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(cryptos) = self.cryptos.clone() {
            html! {
            <div class=classes!("container")>
                <div>
                    <CryptoGeneral name="Bitcoin" image="btc.svg" price=cryptos.bitcoin id="bitcoin"/>
                    <CryptoGeneral name="Ethereum" image="eth.svg" price=cryptos.ethereum id="ethereum"/>
                    <CryptoGeneral name="Chain Link" image="link.svg" price=cryptos.chain_link id ="chainlink"/>
                </div>
                <div>
                    <CryptoGeneral name="Litecoin" image="ltc.svg" price=cryptos.litecoin id = "litecoin"/>
                    <CryptoGeneral name="Bitcoin Cash" image="bch.svg" price=cryptos.bitcoin_cash id ="bitcoin-cash"/>
                    <CryptoGeneral name="Unit Protocol Duck" image="duck.png" price=cryptos.unit_protocol_duck id="unit-protocol-duck"/>
                </div>
            </div>
            }
        } else {
            html! {
                <div class=classes!("container")>
                    {"loading..."}
                </div>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<CryptoAnalyticsApp>::new().mount_to_body();
}
