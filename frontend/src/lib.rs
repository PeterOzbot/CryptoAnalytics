#[macro_use]
extern crate serde;

use chrono::Local;
use serde_json::{from_value, Value};
use std::{time::Duration, vec};
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
use models::{
    crypto::{Crypto, CryptoData},
    price::*,
};

mod common;
mod components;
mod models;

struct CryptoAnalyticsApp {
    link: ComponentLink<Self>,
    cryptos: Option<Vec<CryptoData>>,
    last_updated: Option<chrono::DateTime<Local>>,
    fetch_task: Option<FetchTask>,
    refresh_task: Option<TimeoutTask>,
    crypto_definitions: Vec<Crypto>,
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
            last_updated: None,
            crypto_definitions: vec![
                Crypto {
                    id: "bitcoin",
                    icon: "btc.svg",
                },
                Crypto {
                    id: "ethereum",
                    icon: "eth.svg",
                },
                Crypto {
                    id: "chainlink",
                    icon: "link.svg",
                },
                Crypto {
                    id: "litecoin",
                    icon: "ltc.svg",
                },
                Crypto {
                    id: "bitcoin-cash",
                    icon: "bch.svg",
                },
                Crypto {
                    id: "unit-protocol-duck",
                    icon: "duck.png",
                },
                Crypto {
                    id: "blockstack",
                    icon: "stx.svg",
                },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.cryptos = None;

                // get ids
                let mut ids: String = String::from("");
                for def in &self.crypto_definitions {
                    ids = ids + "," + def.id;
                }

                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/simple/price?ids={:}&vs_currencies=EUR,BTC,ETH&include_24hr_change=true", ids);
                ConsoleService::info(&format!("Loading data: {:?}", url_request));

                // create request
                let req = Request::get(url_request)
                    .body(Nothing)
                    .expect("Loading data failed");

                // callback to handle messaging
                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::Resp(data)
                        });

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);

                // set recurring calls
                self.refresh_task = Some(TimeoutService::spawn(
                    Duration::from_secs(300),
                    self.link.callback(|_| Msg::MakeReq),
                ));
            }
            Msg::Resp(resp) => match resp {
                Ok(data) => {
                    let mut cryptos = Vec::new();

                    for def in &self.crypto_definitions {
                        match from_value::<Price>(data[def.id].clone()) {
                            Ok(price) => {
                                cryptos.push(CryptoData {
                                    definition: def.clone(),
                                    price: price,
                                });
                            }
                            Err(error) => {
                                ConsoleService::info(&format!("Parsing price error: {:}", error));
                            }
                        }
                    }

                    self.cryptos = Some(cryptos);
                    self.last_updated = Some(chrono::offset::Local::now());
                    ConsoleService::info(&format!(
                        "Cryptos: {:?} Time: {:?}",
                        self.cryptos, self.last_updated
                    ));
                }
                Err(error) => {
                    ConsoleService::info(&format!("Message response error: {:}", error));
                }
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let last_updated = match &self.last_updated {
            Some(date) => date.format("%d.%m ~ %H:%M").to_string(),
            None => String::from(""),
        };

        if let Some(cryptos) = &self.cryptos {
            let crypto_html: Vec<Html> = cryptos
                .iter()
                .map(|crypto_data| {
                    html! {
                       <CryptoGeneral price=crypto_data.price.clone() definition=crypto_data.definition.clone()/>
                    }
                })
                .collect();

            html! {
                <div>
                    <div class="page-header">
                        <div class="updated">{"Updated at: "}{last_updated}</div>
                    </div>
                    <div class=classes!("container")>
                        {crypto_html}
                    </div>
                </div>
            }
        } else {
            html! {
                <div>
                    <div class="page-header">{last_updated}</div>
                    <div class=classes!("container")>
                        {"loading..."}
                    </div>
                </div>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<CryptoAnalyticsApp>::new().mount_to_body();
}
