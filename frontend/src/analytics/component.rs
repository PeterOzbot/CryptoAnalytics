use std::{rc::Rc, vec};
use yew::{classes, html, Context, Html};
use yewdux::prelude::Dispatch;

use super::message::Message;
use crate::store::{CryptoState, CryptoStore};

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
        let mut crypto_html: Vec<Html> = vec![html! {
            <div class={classes!("loading-container")}>
                <div class="stage">
                    <div class="dot-carousel"></div>
                </div>
            </div>
        }];

        if let Some(state) = &self.state {
            if let Some(crypto_definitions) = &state.crypto_definitions {
                crypto_html = crypto_definitions
                    .iter()
                    .map(|crypto_definition| {
                        html! {
                           <super::general::Component definition={crypto_definition.clone()}/>
                        }
                    })
                    .collect();
            }
        }

        html! {
            <div class={classes!("analytics-container")}>
                {crypto_html}
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
