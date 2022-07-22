use std::rc::Rc;

use yew::prelude::*;
use yew_agent::Dispatched;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

use crate::agents::AgentRequest;
use crate::agents::LoadAgent;
use crate::analytics;
use crate::portfolio;
use crate::routing::ApplicationRoutes;
use crate::store::{CryptoState, CryptoStore};

use super::message::Message;

pub struct Component {
    _dispatch: Dispatch<CryptoStore>,
    _agent: yew_agent::Dispatcher<LoadAgent>,
    state: Option<Rc<CryptoState>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::bridge_state(ctx.link().callback(Message::State));

        let mut agent = LoadAgent::dispatcher();
        agent.send(AgentRequest::Initialize);

        Self {
            _dispatch: dispatch,
            _agent: agent,
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

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let mut formatted_last_updated = String::from("/ ~ /");

        if let Some(state) = &self.state {
            let last_updated = state.last_updated.clone();
            if let Some(date) = &last_updated {
                formatted_last_updated = date.format("%d.%m ~ %H:%M").to_string();
            }
        }

        html! {
           <div class="main-container">
               <div class="page-header">
                   <div class="updated">{"Updated at: "}{formatted_last_updated}</div>
               </div>

               <div class="page-content">
                <BrowserRouter>
                    <Switch<ApplicationRoutes> render={Switch::render(move |routes| {
                        match routes {
                            ApplicationRoutes::Home => {
                                html! {
                                    <analytics::Component/>
                                }
                            }
                            ApplicationRoutes::Portfolio => {
                                html! {
                                    <portfolio::Component/>
                                }
                            }
                        }
                    })}/>
                </BrowserRouter>
                </div>
            </div>
        }
    }
}
