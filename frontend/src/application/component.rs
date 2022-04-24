use std::time::Duration;

use chrono::Local;
use yew::{
    classes, html,
    services::{timeout::TimeoutTask, ConsoleService, TimeoutService},
    ComponentLink, Html, ShouldRender,
};
use yew_router::router::Router;

use crate::analytics;
use crate::portfolio;
use crate::routing::ApplicationRoutes;

use super::message::Message;

pub struct Component {
    link: ComponentLink<Self>,
    last_updated: Option<chrono::DateTime<Local>>,
    refresh_task: Option<TimeoutTask>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::Refresh);

        Self {
            link,
            refresh_task: None,
            last_updated: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Refresh => {
                // set update time
                self.last_updated = Some(chrono::offset::Local::now());

                // set recurring calls
                self.refresh_task = Some(TimeoutService::spawn(
                    Duration::from_secs(60),
                    self.link.callback(|_| Message::Refresh),
                ));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let last_updated = match &self.last_updated {
            Some(date) => date.format("%d.%m ~ %H:%M").to_string(),
            None => String::from("/ ~ /"),
        };
        ConsoleService::info(&format!("Refresh: {:}", last_updated));

        html! {
           <div class="main-container">
               <div class="page-header">
                   <div class="updated">{"Updated at: "}{last_updated}</div>
               </div>

               <div class="page-content">
                   <Router<ApplicationRoutes, ()>
                       render = Router::render(move |switch: ApplicationRoutes| {
                           match switch {
                               ApplicationRoutes::Home => {
                                   html! {
                                       <analytics::Component />
                                   }
                               }
                               ApplicationRoutes::Portfolio => {
                                   html! {
                                       <portfolio::Component/>
                                   }
                               }
                           }
                       })
                   />
                </div>
            </div>
        }
    }
}
