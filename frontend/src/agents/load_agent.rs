use gloo_console::info;
use gloo_timers::callback::Interval;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use yewdux::prelude::{Dispatch, Dispatcher};

use crate::{
    common::request_get,
    models::{Crypto, GasPrice, GasPriceData, Image, MarketData, Portfolio, Price, PricesData},
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

                // load gas price
                load_gas_price(&self.link);
            }
            Message::DefinitionsLoaded(resp) => match resp {
                Ok(data) => {
                    // notify about definitions
                    info!(&format!(
                        "Load Agent: Definitions -> Loaded data: {:?}",
                        data
                    ));

                    // load prices for each definition
                    load_prices(&data, &self.link);

                    // update state
                    let crypto_definitions = Some(data);
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

                    // load portfolio with price data
                    load_portfolio(id.clone(), &data, &self.link);

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
            Message::PortfolioLoaded(id, resp) => match resp {
                Ok(data) => {
                    info!(&format!(
                        "Load Agent: {:} -> Loaded entries: {:?}",
                        &id, &data
                    ));

                    // update state
                    self.dispatch.reduce(|state: &mut CryptoState| {
                        state.portfolio.remove(&id);
                        state.portfolio.insert(id, data);
                    });
                }
                Err(error) => {
                    info!(&format!(
                        "Load Agent: {:} -> Entries, Response error: {:}",
                        id, error
                    ));
                }
            },
            Message::GasPriceLoaded(resp) => match resp {
                Ok(data) => {
                    info!(&format!(
                        "Load Agent: Gas price -> Loaded data: {:?}",
                        &data
                    ));

                    // update state
                    self.dispatch.reduce(|state: &mut CryptoState| {
                        state.gas_price = Some(data);
                    });
                }
                Err(error) => {
                    info!(&format!(
                        "Load Agent: Gas price -> Response error: {:}",
                        error
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
                    Err(_) => 0,
                };

                if interval_milliseconds == 0 {
                    info!("Load Agent: Reload interval is disabled.");
                    self.link.send_message(Message::Reload);
                    return;
                }

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

fn load_portfolio(api_key: String, price: &PricesData, link: &AgentLink<LoadAgent>) {
    let current_price = price.market_data.current_price.eur;

    link.send_future(async move {
        //url for request
        let url_request = format!(
            "{:}/portfolio/{:}/{:}",
            env!("API_URL"),
            &api_key,
            current_price
        );
        info!(&format!(
            "Load Agent: {:} -> Loading portfolio entries: {:?}",
            &api_key, url_request
        ));

        // get request
        let response = request_get::<Portfolio>(url_request).await;

        // send response
        Message::PortfolioLoaded(api_key, response)
    });
}

fn load_prices(data: &Vec<Crypto>, link: &AgentLink<LoadAgent>) {
    for definition in data {
        let api_key = definition.api_key.clone();
        load_price(api_key, link);
    }
}

fn load_price(api_key: String, link: &AgentLink<LoadAgent>) {
    link.send_future(async move {
        let coingecko_api = env!("COINGECKO_API_KEY");
        if coingecko_api == "****UPDATE****"{
            let response = Ok(create_fake_api_response(&api_key));

            info!(&format!(
                "Load Agent: {:} -> Loading fake prices",
                &api_key
            ));

            // send response
            return Message::PricesLoaded(api_key, response);
        }

        // url for request
        let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false&x_cg_demo_api_key={:}", &api_key, coingecko_api);

        info!(&format!(
            "Load Agent: {:} -> Loading prices: {:?}",
            &api_key, url_request
        ));

        // get request
        let response = request_get::<PricesData>(url_request).await;

        // send response
        Message::PricesLoaded(api_key, response)
    });
}

fn load_gas_price(link: &AgentLink<LoadAgent>) {
    link.send_future(async move {
        let etherscan_api_key = env!("ETHERSCAN_API_KEY");
        if etherscan_api_key == "****UPDATE****" {
            let response = Ok(GasPriceData {
                message: "OK".to_string(),
                result: GasPrice {
                    safe_gas_price: "100".to_string(),
                    propose_gas_price: "100".to_string(),
                    fast_gas_price: "100".to_string(),
                },
            });

            info!(&format!("Load Agent: Loading fake gas price"));

            // send response
            return Message::GasPriceLoaded(response);
        }

        // url for request
        let url_request = format!(
            "https://api.etherscan.io/api?module=gastracker&action=gasoracle&apikey={:}",
            etherscan_api_key
        );

        info!(&format!("Load Agent: Loading gas price: {:?}", url_request));

        // get request
        let response = request_get::<GasPriceData>(url_request).await;

        // send response
        Message::GasPriceLoaded(response)
    });
}

fn create_fake_api_response(api_key: &String) -> PricesData {
    PricesData {
        id: api_key.clone(),
        image: Image {
            thumb: String::new(),
        },
        market_data: MarketData {
            current_price: Price {
                eur: 1000.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_24h_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_percentage_24h_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_percentage_7d_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_percentage_30d_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_percentage_200d_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
            price_change_percentage_1y_in_currency: Price {
                eur: 0.0,
                btc: 0.0,
                eth: 0.0,
            },
        },
    }
}
