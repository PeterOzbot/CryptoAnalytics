use bigdecimal::{BigDecimal, Zero};
use std::{ops::Add, rc::Rc};
use yew::{classes, html, Context, Html};
use yewdux::prelude::Dispatch;

use crate::store::{CryptoState, CryptoStore};

use super::message::Message;

use load_dotenv::load_dotenv;
load_dotenv!();

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
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let mut content_html: Html = html! {
            <div class={classes!("loading-container")}>
                <div class="stage">
                    <div class="dot-carousel"></div>
                </div>
            </div>
        };

        if let Some(state) = &self.state {
            if let Some(crypto_definitions) = &state.crypto_definitions {
                let mut buy_price_sum: BigDecimal = BigDecimal::zero();
                let mut current_price_sum: BigDecimal = BigDecimal::zero();

                for portfolio_entry in state.portfolio.values() {
                    buy_price_sum = buy_price_sum.add(&portfolio_entry.buy_price_sum);
                    current_price_sum = current_price_sum.add(&portfolio_entry.current_price_sum);
                }

                let portfolio_entries_html: Vec<Html> = crypto_definitions
                    .iter()
                    .map(|crypto_definition| {
                        html! {
                        <super::ledger::Component definition={crypto_definition.clone()}/>
                        }
                    })
                    .collect();

                content_html = html! {
                    <div>
                        <div class={classes!("sum-container")}>
                            <div>{"Bought: "}{buy_price_sum}</div>
                            <div>{"Worth: "}{current_price_sum}</div>
                        </div>

                        <div class={classes!("entries-container")}>
                            {portfolio_entries_html}
                        </div>
                    </div>
                };
            }
        }

        html! {
            <div>
                {content_html}
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
