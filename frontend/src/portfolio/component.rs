use bigdecimal::{BigDecimal, Zero};
use std::{ops::Add, rc::Rc};
use yew::{classes, html, Context, Html};
use yewdux::prelude::Dispatch;

use crate::{
    common::FormattedPortfolio,
    store::{CryptoState, CryptoStore},
};

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
                // calculate sums
                let mut purchase_price_sum: BigDecimal = BigDecimal::zero();
                let mut current_price_sum: BigDecimal = BigDecimal::zero();

                for portfolio_entry in state.portfolio.values() {
                    purchase_price_sum =
                        purchase_price_sum.add(&portfolio_entry.purchase_price_sum);
                    current_price_sum = current_price_sum.add(&portfolio_entry.current_price_sum);
                }

                // format sums
                let formatted_sums = FormattedPortfolio::formatted_portfolio(
                    &purchase_price_sum,
                    &current_price_sum,
                );

                // generate definitions components
                let portfolio_entries_html: Vec<Html> = crypto_definitions
                    .iter()
                    .map(|crypto_definition| {
                        html! {
                        <super::ledger::Component definition={crypto_definition.clone()}/>
                        }
                    })
                    .collect();

                // combine whole page html
                content_html = html! {
                    <div class="portfolio-page">
                        <div class="sum-container">
                            <div class="current-value">{formatted_sums.current_value} {"€"}</div>
                            <div class="profit-container">
                                <div class={classes!(&formatted_sums.change_direction,"purchase-value")}>{formatted_sums.purchase_value} {"€"}</div>
                                <div class={classes!(&formatted_sums.change_direction)}>{formatted_sums.change}</div>
                            </div>
                        </div>

                        <div class="entries-container">
                            {portfolio_entries_html}
                        </div>
                    </div>
                };
            }
        }

        html! {
            {content_html}
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
