use std::rc::Rc;
//use web_sys::{EventTarget, HtmlInputElement};
use yew::{html, Context, Event};
use yewdux::prelude::Dispatch;

use crate::{
    models::PricesData,
    store::{CryptoState, CryptoStore},
};

use super::message::Message;

pub struct Component {
    _dispatch: Dispatch<CryptoStore>,
    state: Option<Rc<CryptoState>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

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
            Message::GasLimitChanged(amount) => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let mut content = html! {
                <div class="loading-info">
                    <div class="stage">
                        <div class="dot-carousel"></div>
                    </div>
                </div>
        };

        if let Some(state) = &self.state {
            if let Some(prices_data) = state.crypto_prices.get("ethereum") {
                if let Some(gas_price) = &state.gas_price {
                    let on_dangerous_change = ctx.link().callback(|e: Event| {
                        // let target: EventTarget = e
                        //     .target()
                        //     .expect("Event should have a target when dispatched");
                        // // You must KNOW target is a HtmlInputElement, otherwise
                        // // the call to value would be Undefined Behaviour (UB).
                        // Message::GasLimitChanged(
                        //     target.unchecked_into::<HtmlInputElement>().value(),
                        // )
                        Message::GasLimitChanged(String::from("10"))
                    });

                    content = html! {
                        <div class="values-container">
                            <div class="input">
                            <input onchange={on_dangerous_change} type="number" />
                            </div>
                            <div class="value">
                                <div class="title">
                                    {"Low "}
                                </div>
                                <div class="price">
                                    <div class="gas-price">
                                        {&gas_price.result.safe_gas_price}{" gwei"}
                                    </div>
                                    <div class="eur-price">
                                        {calculate_price(prices_data, &gas_price.result.safe_gas_price)}
                                    </div>
                                </div>
                            </div>
                            <div class="value">
                            <div class="title">
                                    {"Average"}
                                </div>
                                <div class="price">
                                <div class="gas-price">
                                    {&gas_price.result.propose_gas_price}{" gwei"}
                                </div>
                                <div class="eur-price">
                                    {calculate_price(prices_data, &gas_price.result.propose_gas_price)}
                                </div>
                            </div>
                            </div>
                            <div class="value">
                            <div class="title">
                                    {"High"}
                                </div>
                                <div class="price">
                                <div class="gas-price">
                                    {&gas_price.result.fast_gas_price}{" gwei"}
                                </div>
                                <div class="eur-price">
                                    {calculate_price(prices_data, &gas_price.result.fast_gas_price)}
                                </div>
                            </div>
                            </div>
                        </div>
                    };
                }
            }
        }

        html! {
            <div class="gas-row">
                {content}
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}

fn calculate_price(prices_data: &PricesData, gas_price: &String) -> String {
    let gas_price = gas_price.parse::<f64>();
    match gas_price {
        Ok(gas_price) => {
            // Gas fees are calculated by multiplying the gas price by the gas limit. ‌So, if the gas limit is 21,000 and the price per unit is 200 gwei, the fee would be 21,000 * 200 = 4,000,000 gwei or 0.004 ETH.
            let gas_price_in_eth = (gas_price * 21000.0) / 1000000000.0;

            let eth_price = prices_data.market_data.current_price.eur;
            let gas_price_in_eur = gas_price_in_eth * eth_price;

            format!("{:.2} EUR", gas_price_in_eur)
        }
        Err(_) => {
            return String::from("0.00 EUR");
        }
    }
}
