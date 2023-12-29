use yew::html::Scope;
use yew::{html, Html};

use crate::analytics;
use crate::etherscan;
use crate::portfolio;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

use super::message::Message;
use super::Component;

#[derive(Debug, Display, EnumIter, Clone, Copy)]
pub enum Tab {
    Analytics,
    Portfolio,
    Etherscan,
}

pub fn get_tab_content(tab: &Tab) -> Html {
    match tab {
        Tab::Analytics => html! {
            <analytics::Component />
        },
        Tab::Portfolio => html! {
            <portfolio::Component />
        },
        Tab::Etherscan => html! {
            <etherscan::Component />
        },
    }
}

pub fn get_tab_buttons(link: &Scope<Component>, selected_tab: &Option<Tab>) -> Html {

let buttons:Html = Tab::iter().map(|tab| { 

    let mut selected_tab_key = "";
    if let Some(selected) = selected_tab {
       if selected.to_string() == tab.to_string() {
           selected_tab_key = "selected";
       }
    }

    html! {
        <div class={selected_tab_key.to_string()} onclick={link.callback(move |_| Message::TabChange(tab))}>
            { &tab.to_string() }
        </div>
    }
}).collect();

html!{
    <div class="tabs">
        {buttons}
    </div>
}
}
