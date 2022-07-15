use chrono::Local;
use gloo_console::info;
use gloo_timers::callback::Timeout;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::analytics;
use crate::portfolio;
use crate::routing::ApplicationRoutes;

use super::message::Message;

pub struct Component {
    last_updated: Option<chrono::DateTime<Local>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Message::Refresh);

        Self { last_updated: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Refresh => {
                // set update time
                self.last_updated = Some(chrono::offset::Local::now());

                // set timer update
                let callback = ctx.link().callback(|_| Message::Refresh);
                let timeout = Timeout::new(300_000, move || callback.emit(()));
                //let timeout = Timeout::new(5000, move || callback.emit(()));

                // Since we don't plan on cancelling the timeout, call `forget`.
                timeout.forget();
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let last_updated = self.last_updated.clone();
        let formatted_last_updated = match &last_updated {
            Some(date) => date.format("%d.%m ~ %H:%M").to_string(),
            None => String::from("/ ~ /"),
        };
        info!(&format!("Refresh: {:}", formatted_last_updated.clone()));

        let html = html! {
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
                                    <analytics::Component last_updated={last_updated}/>
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
        };

        html
    }
}
