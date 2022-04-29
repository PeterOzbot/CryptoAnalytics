use std::vec;
use yew::{
    classes,
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    ComponentLink, Html, ShouldRender,
};

use super::message::Message;
use crate::models::Crypto;

use load_dotenv::load_dotenv;
load_dotenv!();

pub struct Component {
    link: ComponentLink<Self>,
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
            fetch_task: None,
            crypto_definitions: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::LoadDefinitions => {
                self.crypto_definitions = None;

                // url for request
                let url_request = format!("{:}/definitions", env!("API_URL"));
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
                }
                Err(error) => {
                    ConsoleService::info(&format!(
                        "Definitions -> Message response error: {:}",
                        error
                    ));
                }
            },
        }
        true
    }

    fn view(&self) -> Html {
        let crypto_html: Vec<Html>;
        if let Some(crypto_definitions) = &self.crypto_definitions {
            crypto_html = crypto_definitions
                .iter()
                .map(|crypto_definition| {
                    html! {
                       <super::general::Component definition=crypto_definition.clone()/>
                    }
                })
                .collect();
        } else {
            crypto_html = vec![html! {
                <div class=classes!("loading-container")>
                    <div class="stage">
                        <div class="dot-carousel"></div>
                    </div>
                </div>
            }];
        }

        html! {
            <div class=classes!("analytics-container")>
                {crypto_html}
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.link.send_message(Message::LoadDefinitions);
        false
    }
}
