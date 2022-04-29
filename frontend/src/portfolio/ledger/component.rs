use std::ops::{Add, Mul, Sub};

use bigdecimal::{BigDecimal, Zero};
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    ComponentLink, Html, ShouldRender,
};

use crate::models::{Crypto, Entry};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}
pub struct Component {
    properties: Properties,
    data: Option<Vec<Entry>>,
    link: yew::ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(super::message::Message::LoadEntries);
        Self {
            properties,
            data: None,
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::LoadEntries => {
                self.data = None;

                // url for request
                let url_request = format!(
                    "{:}/definition/{:}",
                    env!("API_URL"),
                    self.properties.definition.api_key
                );
                ConsoleService::info(&format!(
                    "{:} -> Loading entries: {:?}",
                    self.properties.definition.api_key, url_request
                ));

                // create request
                let req = Request::get(&url_request).body(Nothing).expect(
                    format!(
                        "Loading entries data for {:} failed.",
                        self.properties.definition.api_key
                    )
                    .as_str(),
                );

                // callback to handle messaging
                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Entry>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Message::EntriesLoaded(data)
                    },
                );

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect(&format!(
                    "{:} -> Entries, Fetch failed: {:?}",
                    self.properties.definition.api_key, url_request
                ));
                self.fetch_task = Some(task);
            }
            Message::EntriesLoaded(resp) => match resp {
                Ok(data) => {
                    self.data = Some(data);
                    ConsoleService::info(&format!(
                        "{:} -> Loaded entries: {:?}",
                        self.properties.definition.api_key, self.data
                    ));
                }
                Err(error) => {
                    ConsoleService::info(&format!(
                        "{:} -> Entries, response error: {:}",
                        self.properties.definition.api_key, error
                    ));
                }
            },
        }
        true
    }

    fn view(&self) -> Html {
        let crypto_key = &self.properties.definition.api_key;

        if let Some(data) = &self.data {
            let mut sum_amount: BigDecimal = BigDecimal::zero();
            let mut sum_price: BigDecimal = BigDecimal::zero();

            let mut entries_html: Vec<Html> = vec![];
            for entry in data.iter() {
                sum_amount = sum_amount.add(&entry.amount);

                let amount = &entry.amount.clone().sub(&entry.withdraw_fee);
                let price = amount.mul(&entry.price);
                let final_price = price.sub(&entry.purchase_fee);
                sum_price = sum_price.add(final_price);

                entries_html.push(html! {
                   <div class="ledger-entry">
                        <div class="amount">{&entry.amount}</div>
                        <div class="price">{&entry.price}</div>
                   </div>
                });
            }

            yew::html! {
                <div class="ledger-column">
                    <div>{crypto_key}</div>
                    <div>{entries_html}</div>
                    <div class="ledger-sum">
                        <div class="amount">{sum_amount}</div>
                        <div class="price">{sum_price}</div>
                    </div>
                </div>
            }
        } else {
            // loading indicator
            yew::html! {
                <div class="ledger-column">
                    <div>{crypto_key}</div>
                    <div class="loading-info">
                        <div class="stage">
                            <div class="dot-carousel"></div>
                        </div>
                    </div>
                </div>
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.link.send_message(Message::LoadEntries);
        false
    }
}
