use std::rc::Rc;

use yew::prelude::*;
use yew_agent::Dispatched;
use yewdux::prelude::Dispatch;

use crate::agents::AgentRequest;
use crate::agents::LoadAgent;
use crate::application::tab::get_tab_buttons;
use crate::application::tab::get_tab_content;
use crate::store::{CryptoState, CryptoStore};

use super::message::Message;
use super::tab::Tab;

pub struct Component {
    _dispatch: Dispatch<CryptoStore>,
    _agent: yew_agent::Dispatcher<LoadAgent>,
    state: Option<Rc<CryptoState>>,
    tab: Option<Tab>,
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
            tab: Some(Tab::Analytics),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::State(state) => {
                self.state = Some(state);
            }
            Message::TabChange(tab) => {
                self.tab = Some(tab);
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

        let mut tab_content: Html = html! {};
        if let Some(tab) = &self.tab {
            tab_content = get_tab_content(tab);
        }

        let link: html::Scope<Component> = _ctx.link().clone();
        let tab_buttons = get_tab_buttons(&link, &self.tab);

        // let mut tab_content: Html = html! {};
        // let link = _ctx.link().clone();
        // tab_content = html! {
        //     <div class="tabs">
        //         {get_tab_button(&link, &Tab::Analytics, "selected")}
        //         {get_tab_button(&link, &Tab::Portfolio, "")}
        //         {get_tab_button(&link, &Tab::Etherscan, "")}
        //     </div>
        // };

        html! {
           <div class="main-container">
               <div class="page-header">
                <div class="updated">{"Updated at: "}{formatted_last_updated}</div>
                    {tab_buttons}
               </div>

               <div class="page-content">
                    {tab_content}
               </div>
            </div>
        }
    }
}
