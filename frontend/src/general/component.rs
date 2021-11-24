use yew::{
    format::{Json, Nothing},
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

use crate::models::crypto::Crypto;

use super::{data::Data, message::Message};

#[derive(yew::Properties, Clone, PartialEq)]
pub struct Properties {
    pub definition: Crypto,
}

pub struct Component {
    properties: Properties,
    data: Option<Data>,
    link: yew::ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

impl yew::Component for Component {
    type Properties = Properties;
    type Message = super::message::Message;

    fn create(properties: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        link.send_message(super::message::Message::MakeReq);
        Self {
            properties,
            data: None,
            link,
            fetch_task: None,
        }
    }

    fn view(&self) -> yew::Html {
        if let Some(data) = &self.data {

            // construct HTML
            yew::html! {
                <div class="general-row">
                    <img src=data.image.thumb.clone()/>
                    <div>{data.market_data.current_price.eur}</div>
                </div>
            }
        } else {
            // loading indicator
            yew::html! {
                <div>
                    {"loading..."}
                </div>
            }
        }
    }
    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Message::MakeReq => {
                self.data = None;

                // url for request
                let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",self.properties.definition.id);
                ConsoleService::info(&format!("Loading data: {:?}", url_request));

                // create request
                let req = Request::get(url_request).body(Nothing).expect(
                    format!(
                        "Loading general data for {:} failed.",
                        self.properties.definition.id
                    )
                    .as_str(),
                );

                // callback to handle messaging
                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<Data, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Message::Resp(data)
                        });

                // set task to avoid out of scope
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
            }
            Message::Resp(resp) => match resp {
                Ok(data) => {
                    self.data = Some(data);
                    ConsoleService::info(&format!(
                        "Loaded for {:} Data: {:?}",
                        self.properties.definition.id, self.data
                    ));
                }
                Err(error) => {
                    ConsoleService::info(&format!("Message response error: {:}", error));
                }
            },
        }
        true
    }

    fn change(&mut self, properties: Self::Properties) -> yew::ShouldRender {
        self.properties = properties;
        true
    }
}
