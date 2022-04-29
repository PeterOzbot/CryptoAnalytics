#[macro_use]
extern crate serde;

mod analytics;
mod application;
mod common;
mod models;
mod portfolio;
mod routing;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<application::Component>::new().mount_as_body();
}
