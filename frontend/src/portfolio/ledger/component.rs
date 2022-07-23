use bigdecimal::{BigDecimal, Zero};
use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};
use yew::{html, Context, Html};
use yewdux::prelude::Dispatch;

use crate::{
    models::Crypto,
    store::{CryptoState, CryptoStore},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}
pub struct Component {
    _dispatch: Dispatch<CryptoStore>,
    state: Option<Rc<CryptoState>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(Message::State));

        Self {
            _dispatch: dispatch,
            state: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::State(state) => {
                self.state = Some(state);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let crypto_key = &_ctx.props().definition.api_key;

        if let Some(state) = &self.state {
            if let Some(data) = state.portfolio.get(crypto_key) {
                let mut sum_amount: BigDecimal = BigDecimal::zero();
                let mut sum_price: BigDecimal = BigDecimal::zero();

                let mut entries_html: Vec<Html> = vec![];
                for entry in data {
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

                return yew::html! {
                    <div class="ledger-column">
                        <div>{crypto_key}</div>
                        <div>{entries_html}</div>
                        <div class="ledger-sum">
                            <div class="amount">{sum_amount}</div>
                            <div class="price">{sum_price}</div>
                        </div>
                    </div>
                };
            }
        }

        return yew::html! {
            <div class="ledger-column">
                <div>{crypto_key}</div>
                <div class="loading-info">
                    <div class="stage">
                        <div class="dot-carousel"></div>
                    </div>
                </div>
            </div>
        };
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
