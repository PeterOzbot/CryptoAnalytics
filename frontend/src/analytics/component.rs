use chrono::Local;
use std::{time::Duration, vec};
use yew::{
    classes,
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        timeout::TimeoutTask,
        ConsoleService, FetchService, TimeoutService,
    },
    ComponentLink, Html, ShouldRender,
};

use crate::models::Crypto;

use super::message::Message;

use load_dotenv::load_dotenv;
load_dotenv!();

pub struct Component {
    link: ComponentLink<Self>,
    last_updated: Option<chrono::DateTime<Local>>,
    refresh_task: Option<TimeoutTask>,
    fetch_task: Option<FetchTask>,
    crypto_definitions: Option<Vec<Crypto>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::LoadDefinitions);

        Self {
            link,
            refresh_task: None,
            fetch_task: None,
            last_updated: None,
            crypto_definitions: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::LoadDefinitions => {
                self.crypto_definitions = None;

                // url for request
                let url_request = format!("{:}{:}", env!("API_URL"), "/definitions");
                ConsoleService::info(&format!("Definitions -> Loading data: {:?}", url_request));

                // create request
                let req = Request::get(&url_request)
                    .body(Nothing)
                    .expect("Loading Definitions failed.");

                // callback to handle messaging
                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Crypto>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Message::DefinitionsLoaded(data)
                    },
                );

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb)
                    .expect(&format!("Definitions -> Fetch failed: {:?}", url_request));
                self.fetch_task = Some(task);
            }
            Message::DefinitionsLoaded(resp) => match resp {
                Ok(data) => {
                    self.crypto_definitions = Some(data);
                    ConsoleService::info(&format!(
                        "Definitions -> Loaded data: {:?}",
                        self.crypto_definitions
                    ));
                    self.link.send_message(Message::Refresh);
                }
                Err(error) => {
                    ConsoleService::info(&format!(
                        "Definitions -> Message response error: {:}",
                        error
                    ));
                }
            },
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
            None => String::from("no date"),
        };
        ConsoleService::info(&format!("Refresh: {:}", last_updated));

        let crypto_html: Vec<Html>;
        if let Some(crypto_definitions) = &self.crypto_definitions {
            crypto_html = crypto_definitions
                .iter()
                .map(|crypto_definition| {
                    html! {
                       <crate::general::Component definition=crypto_definition.clone()/>
                    }
                })
                .collect();
        } else {
            crypto_html = vec![html! {
                <div class=classes!("loading-container")>
                    {"....loading..."}
                </div>
            }];
        }

        html! {
            <div>
                <div class="page-header">
                    <div class="updated">{"Updated at: "}{last_updated}</div>
                </div>
                <div class=classes!("general-container")>
                    {crypto_html}
                </div>
            </div>
        }
    }
}
