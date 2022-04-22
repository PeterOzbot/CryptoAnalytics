#[macro_use]
extern crate serde;

mod analytics;
mod common;
mod general;
mod history;
mod models;

use models::Crypto;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<analytics::Component>::new().mount_to_body_with_props(analytics::Properties {
        crypto_definitions: vec![
            Crypto {
                api_key: String::from("bitcoin"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("ethereum"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("chainlink"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("litecoin"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("bitcoin-cash"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("blockstack"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("defichain"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("binancecoin"),
                precision: 2,
            },
            Crypto {
                api_key: String::from("unit-protocol-duck"),
                precision: 4,
            },
        ],
    });
}
