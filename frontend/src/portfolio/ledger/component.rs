use std::rc::Rc;
use yew::{classes, Context};
use yewdux::prelude::Dispatch;

use crate::{
    common::FormattedPortfolio,
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
                if let Some(price) = state.crypto_prices.get(crypto_key) {
                    // format amounts
                    let formatted_amounts = FormattedPortfolio::formatted_portfolio(
                        &data.purchase_price_sum,
                        &data.current_price_sum,
                    );

                    return yew::html! {
                        <div class="ledger-row-container">
                            <div class="ledger-row">
                                <img alt={crypto_key.clone()} src={price.image.thumb.clone()}/>

                                <div class="amount">{&data.amount_sum}</div>

                                <div class="price">
                                    <div class="current_price">{formatted_amounts.current_value} {"€"}</div>
                                    <div class="profit-container">
                                        <div class={classes!(&formatted_amounts.change_direction,"purchase-value")}>{formatted_amounts.purchase_value} {"€"}</div>
                                        <div class={classes!(&formatted_amounts.change_direction)}>{formatted_amounts.change}</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    };
                }
            }
        }

        return yew::html! {
            <div class="ledger-row-container">
                <div class="ledger-row">
                    <div class="loading-info">
                        <div class="stage">
                            <div class="dot-carousel"></div>
                        </div>
                    </div>
                </div>
            </div>
        };
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
