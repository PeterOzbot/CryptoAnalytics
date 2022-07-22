use gloo_console::info;
use gloo_timers::callback::Interval;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use yewdux::prelude::{Dispatch, Dispatcher};

use crate::{
    common::request_get,
    models::{Crypto, PricesData},
    store::{CryptoState, CryptoStore},
};

use super::{message::Message, AgentRequest};

use load_dotenv::load_dotenv;
load_dotenv!();

pub struct LoadAgent {
    link: AgentLink<LoadAgent>,
    interval: Option<Interval>,
    dispatch: Dispatch<CryptoStore>,
}

impl Agent for LoadAgent {
    type Reach = Context<Self>;
    type Message = Message;
    type Input = AgentRequest;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        let dispatch = Dispatch::<CryptoStore>::new();

        Self {
            link,
            interval: None,
            dispatch,
        }
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Reload => {
                self.dispatch.reduce(|state: &mut CryptoState| {
                    state.last_updated = Some(chrono::offset::Local::now())
                });
                let formatted_last_updated = chrono::offset::Local::now()
                    .format("%d.%m ~ %H:%M")
                    .to_string();
                info!(&format!("Load Agent: Refresh: {:}", formatted_last_updated));

                // load definitions
                self.link.send_future(async move {
                    // url for request
                    let url_request = format!("{:}/definitions", env!("API_URL"));
                    info!(&format!("Load Agent: Loading data: {:?}", url_request));

                    let response = request_get::<Vec<Crypto>>(url_request).await;

                    Message::DefinitionsLoaded(response)
                });
            }
            Message::DefinitionsLoaded(resp) => match resp {
                Ok(data) => {
                    // load prices for each definition
                    load_prices(&data, &self.link);

                    // notify about definitions
                    let crypto_definitions = Some(data);
                    info!(&format!(
                        "Load Agent: Definitions -> Loaded data: {:?}",
                        crypto_definitions
                    ));

                    // update state
                    self.dispatch.reduce(|state: &mut CryptoState| {
                        state.crypto_definitions = crypto_definitions;
                    });
                }
                Err(error) => {
                    info!(&format!(
                        "Load Agent: Definitions -> Message response error: {:}",
                        error
                    ));
                }
            },
            Message::PricesLoaded(id, resp) => match resp {
                Ok(data) => {
                    info!(&format!("Load Agent: {:} -> Loaded data: {:?}", &id, &data));

                    self.dispatch.reduce(|state: &mut CryptoState| {
                        state.crypto_prices.remove(&id);
                        state.crypto_prices.insert(id, data);
                    });
                }
                Err(error) => {
                    info!(&format!(
                        "Load Agent: {:} -> Response error: {:}",
                        id, error
                    ));
                }
            },
        }
    }

    fn handle_input(&mut self, request: Self::Input, _id: HandlerId) {
        match request {
            AgentRequest::Initialize => {
                let interval_milliseconds = match env!("RELOAD_INTERVAL").parse::<u32>() {
                    Ok(value) => value,
                    Err(_) => 2 * 60 * 1000,
                };

                info!(&format!(
                    "Load Agent: Starting reload interval. Interval in milliseconds: {:}",
                    interval_milliseconds
                ));
                self.interval = Some({
                    let link = self.link.clone();
                    Interval::new(interval_milliseconds, move || {
                        link.send_message(Message::Reload);
                    })
                });
                self.link.send_message(Message::Reload);
            }
        }
    }

    fn connected(&mut self, _: HandlerId) {}

    fn disconnected(&mut self, _: HandlerId) {}
}

fn load_prices(data: &Vec<Crypto>, link: &AgentLink<LoadAgent>) {
    for definition in data {
        let api_key = definition.api_key.clone();
        load_price(api_key, link);
    }
}

fn load_price(api_key: String, link: &AgentLink<LoadAgent>) {
    link.send_future(async move {

        // url for request
        let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false", &api_key);

        info!(&format!(
            "Load Agent: {:} -> Loading data: {:?}",
            &api_key, url_request
        ));

        // get request
        let response = request_get::<PricesData>(url_request).await;

        // send response
        Message::PricesLoaded(api_key,response)
    });
}
