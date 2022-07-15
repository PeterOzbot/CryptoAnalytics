use gloo_console::info;
use yew::{classes, html, Context, Html};

use crate::{common::request_get, models::Crypto};

use super::message::Message;

use load_dotenv::load_dotenv;
load_dotenv!();

pub struct Component {
    crypto_definitions: Option<Vec<Crypto>>,
}

impl yew::Component for Component {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Message::LoadDefinitions);
        Self {
            //fetch_task: None,
            crypto_definitions: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::LoadDefinitions => {
                self.crypto_definitions = None;

                ctx.link().send_future(async move {
                    // url for request
                    let url_request = format!("{:}/definitions", env!("API_URL"));
                    info!(&format!(
                        "Portfolio component: Loading data: {:?}",
                        url_request
                    ));

                    let response = request_get::<Vec<Crypto>>(url_request).await;

                    Message::DefinitionsLoaded(response)
                });
            }
            Message::DefinitionsLoaded(resp) => match resp {
                Ok(data) => {
                    self.crypto_definitions = Some(data);
                    info!(&format!(
                        "Portfolio component: Definitions -> Loaded data: {:?}",
                        self.crypto_definitions
                    ));
                }
                Err(error) => {
                    info!(&format!(
                        "Portfolio component: Definitions -> Message response error: {:}",
                        error
                    ));
                }
            },
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> yew::Html {
        let crypto_html: Vec<Html>;
        if let Some(crypto_definitions) = &self.crypto_definitions {
            crypto_html = crypto_definitions
                .iter()
                .map(|crypto_definition| {
                    html! {
                       <super::ledger::Component definition={crypto_definition.clone()}/>
                    }
                })
                .collect();
        } else {
            crypto_html = vec![html! {
                <div class={classes!("loading-container")}>
                    <div class="stage">
                        <div class="dot-carousel"></div>
                    </div>
                </div>
            }];
        }

        html! {
            <div class={classes!("portfolio-container")}>
                {crypto_html}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(Message::LoadDefinitions);
        false
    }
}
