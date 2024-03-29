#[macro_use]
extern crate serde;

mod agents;
mod analytics;
mod application;
mod common;
mod etherscan;
mod models;
mod portfolio;
mod store;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<application::Component>();
}
