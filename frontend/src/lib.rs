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
mod general;
mod models;

struct CryptoAnalyticsApp {
    link: ComponentLink<Self>,
    last_updated: Option<chrono::DateTime<Local>>,
    refresh_task: Option<TimeoutTask>,
    crypto_definitions: Vec<Crypto>,
}

impl Component for CryptoAnalyticsApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Refresh);

        Self {
            link,
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
                // Crypto {
                //     id: "chainlink",
                //     icon: "link.svg",
                // },
                // Crypto {
                //     id: "litecoin",
                //     icon: "ltc.svg",
                // },
                // Crypto {
                //     id: "bitcoin-cash",
                //     icon: "bch.svg",
                // },
                // Crypto {
                //     id: "unit-protocol-duck",
                //     icon: "duck.png",
                // },
                // Crypto {
                //     id: "blockstack",
                //     icon: "stx.svg",
                // },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Refresh => {
                // set update time
                self.last_updated = Some(chrono::offset::Local::now());

                // set recurring calls
                self.refresh_task = Some(TimeoutService::spawn(
                    Duration::from_secs(60),
                    self.link.callback(|_| Msg::Refresh),
                ));
            }
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

        let crypto_html: Vec<Html> = self
            .crypto_definitions
            .iter()
            .map(|crypto_definition| {
                html! {
                   <general::Component definition=crypto_definition.clone()/>
                }
            })
            .collect();

        html! {
            <div>
                <div class="page-header">
                    <div class="updated">{"Updated at: "}{last_updated}</div>
                </div>
                <div class=classes!("general-container")>
                    {crypto_html}
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<CryptoAnalyticsApp>::new().mount_to_body();
}
