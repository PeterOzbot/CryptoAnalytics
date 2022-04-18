#[macro_use]
extern crate serde;

mod analytics;
mod common;
mod general;
mod history;
mod models;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<analytics::Component>::new().mount_to_body();
}
