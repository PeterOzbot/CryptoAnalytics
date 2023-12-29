use std::rc::Rc;
use yew::{html, Context};
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
        html! {
            <div class="etherscan-page">
               <super::gas::Component/>
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        false
    }
}
