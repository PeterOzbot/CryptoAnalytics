#[macro_use]
extern crate serde;

use chrono::Local;
use std::{time::Duration, vec};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{
    classes, html,
    services::{timeout::TimeoutTask, TimeoutService},
    App, Component, ComponentLink, Html, ShouldRender,
};

use models::{crypto::Crypto, price::*};

mod common;
mod general;
mod history;
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
                    id: String::from("bitcoin"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("ethereum"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("chainlink"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("litecoin"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("bitcoin-cash"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("blockstack"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("defichain"),
                    precision: 2,
                },
                Crypto {
                    id: String::from("unit-protocol-duck"),
                    precision: 4,
                },
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
