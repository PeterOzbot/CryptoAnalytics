use std::ops::{Add, Mul, Sub};

use bigdecimal::{BigDecimal, Zero};
use gloo_console::info;
use yew::{html, Context, Html};

use crate::{
    common::request_get,
    models::{Crypto, Entry},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}
pub struct Component {
    data: Option<Vec<Entry>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_message(super::message::Message::LoadEntries);
        Self { data: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let properties = ctx.props().clone();
        match msg {
            Message::LoadEntries => {
                self.data = None;

                ctx.link().send_future(async move {
                    // url for request
                    let url_request = format!(
                        "{:}/definition/{:}",
                        env!("API_URL"),
                        properties.definition.api_key
                    );
                    info!(&format!(
                        "Ledger component: {:} -> Loading entries: {:?}",
                        properties.definition.api_key, url_request
                    ));

                    let response = request_get::<Vec<Entry>>(url_request).await;

                    Message::EntriesLoaded(response)
                });
            }
            Message::EntriesLoaded(resp) => match resp {
                Ok(data) => {
                    self.data = Some(data);
                    info!(&format!(
                        "Ledger component: {:} -> Loaded entries: {:?}",
                        properties.definition.api_key, self.data
                    ));
                }
                Err(error) => {
                    info!(&format!(
                        "Ledger component: {:} -> Entries, response error: {:}",
                        properties.definition.api_key, error
                    ));
                }
            },
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let crypto_key = &_ctx.props().definition.api_key;

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

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(Message::LoadEntries);
        false
    }
}
