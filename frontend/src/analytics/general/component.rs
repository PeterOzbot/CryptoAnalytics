use gloo_console::info;
use yew::{
    classes,
    function_component,
    //format::{Json, Nothing},
    html,
    use_effect,
    use_effect_with_deps,
    use_state,
    Callback,
    Html,
    Properties,
};
use yew_agent::Bridged;
use yew_hooks::{use_async, use_async_with_options, use_effect_update, UseAsyncOptions};

use crate::{
    agents::EventBus,
    common::{request_get, FormattedPrice},
    models::{Crypto, PricesData},
};

use super::message::Message;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct GeneralProperties {
    pub definition: Crypto,
}

#[function_component(General)]
pub fn general(props: &GeneralProperties) -> Html {
    info!("~General component~");

    let properties = props.clone();
    let definition = props.definition.clone();

    let prices_data = use_async_with_options(
        async move {
            let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",definition.api_key);

            info!(&format!(
                "General component: {:} -> Loading data: {:?}",
                definition.api_key, url_request
            ));

            request_get::<PricesData>(url_request).await
        },
        UseAsyncOptions { auto: true },
    );

    // let interval = {
    //     let prices_data = prices_data.clone();
    //     Callback::from(move |_| {
    //         let prices_data = prices_data.clone();
    //         prices_data.run();
    //     })
    // };

    // use_effect(move || {
    //     let producer = EventBus::bridge(interval);

    //     || drop(producer)
    // });

    // let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",properties.definition.api_key);
    //                 info!(&format!(
    //                     "{:} -> Loading data: {:?}",
    //                     properties.definition.api_key, url_request
    //                 ));

    if let Some(data) = &prices_data.data {
        let formatted_price = FormattedPrice::format_data(data, properties.definition.precision);

        // construct HTML
        yew::html! {
            <div class="general-row">
                <div class="general-info">
                    <img src={data.image.thumb.clone()}/>
                    <div class="general-price">
                        <div class={classes!(&formatted_price.change_direction, "price")}>{formatted_price.value}</div>
                        <div class={classes!(&formatted_price.change_direction, "change")}>{formatted_price.change}</div>
                    </div>
                </div>

                <crate::analytics::history::Component label="24h" price_change={data.market_data.price_change_percentage_24h_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=true />

                <crate::analytics::history::Component label="7d" price_change={data.market_data.price_change_percentage_7d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                <crate::analytics::history::Component label="30d" price_change={data.market_data.price_change_percentage_30d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                <crate::analytics::history::Component label="200d" price_change={data.market_data.price_change_percentage_200d_in_currency.clone()}current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                <crate::analytics::history::Component label="1y" price_change={data.market_data.price_change_percentage_1y_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

                <div class="legend">
                    <div>{"EUR"}</div>
                    <div>{"BTC"}</div>
                    <div>{"ETH"}</div>
                </div>
            </div>
        }
    } else {
        // loading indicator
        yew::html! {
            <div class="general-row">
            <div class="loading-info">
                <div class="stage">
                    <div class="dot-carousel"></div>
                </div>
            </div>
        </div>
        }
    }
}

// pub struct Component {
//     data: Option<PricesData>,
//     //fetch_task: Option<FetchTask>,
// }

// impl yew::Component for Component {
//     type Properties = Properties;
//     type Message = super::message::Message;

//     fn create(ctx: &Context<Self>) -> Self {
//         ctx.link().send_message(super::message::Message::LoadPrices);
//         Self {
//             data: None,
//             //fetch_task: None,
//         }
//     }

//     fn view(&self, _ctx: &Context<Self>) -> yew::Html {
//         let properties = _ctx.props();
//         if let Some(data) = &self.data {
//             let formatted_price =
//                 FormattedPrice::format_data(data, properties.definition.precision);

//             // construct HTML
//             yew::html! {
//                 <div class="general-row">
//                     <div class="general-info">
//                         <img src={data.image.thumb.clone()}/>
//                         <div class="general-price">
//                             <div class={classes!(&formatted_price.change_direction, "price")}>{formatted_price.value}</div>
//                             <div class={classes!(&formatted_price.change_direction, "change")}>{formatted_price.change}</div>
//                         </div>
//                     </div>

//                     <crate::analytics::history::Component label="24h" price_change={data.market_data.price_change_percentage_24h_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=true />

//                     <crate::analytics::history::Component label="7d" price_change={data.market_data.price_change_percentage_7d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

//                     <crate::analytics::history::Component label="30d" price_change={data.market_data.price_change_percentage_30d_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

//                     <crate::analytics::history::Component label="200d" price_change={data.market_data.price_change_percentage_200d_in_currency.clone()}current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

//                     <crate::analytics::history::Component label="1y" price_change={data.market_data.price_change_percentage_1y_in_currency.clone()} current_price={data.market_data.current_price.clone()} definition={properties.definition.clone()} use_absolute=false/>

//                     <div class="legend">
//                         <div>{"EUR"}</div>
//                         <div>{"BTC"}</div>
//                         <div>{"ETH"}</div>
//                     </div>
//                 </div>
//             }
//         } else {
//             // loading indicator
//             yew::html! {
//                 <div class="general-row">
//                 <div class="loading-info">
//                     <div class="stage">
//                         <div class="dot-carousel"></div>
//                     </div>
//                 </div>
//             </div>
//             }
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         let properties = ctx.props();
//         match msg {
//             Message::LoadPrices => {
//                 self.data = None;

//                 // url for request
//                 let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",properties.definition.api_key);
//                 info!(&format!(
//                     "{:} -> Loading data: {:?}",
//                     properties.definition.api_key, url_request
//                 ));

//                 // // create request
//                 // let req = Request::get(&url_request).body(Nothing).expect(
//                 //     format!(
//                 //         "Loading general data for {:} failed.",
//                 //         properties.definition.api_key
//                 //     )
//                 //     .as_str(),
//                 // );

//                 // // callback to handle messaging
//                 // let cb = ctx.link().callback(
//                 //     |response: Response<Json<Result<PricesData, anyhow::Error>>>| {
//                 //         let Json(data) = response.into_body();
//                 //         Message::PricesLoaded(data)
//                 //     },
//                 // );

//                 // // set task to avoid out of scope
//                 // let task = FetchService::fetch(req, cb).expect(&format!(
//                 //     "{:} -> General, Fetch failed: {:?}",
//                 //     properties.definition.api_key, url_request
//                 // ));
//                 // self.fetch_task = Some(task);
//             }
//             Message::PricesLoaded(resp) => match resp {
//                 Ok(data) => {
//                     self.data = Some(data);
//                     info!(&format!(
//                         "{:} -> Loaded data: {:?}",
//                         properties.definition.api_key, self.data
//                     ));
//                 }
//                 Err(error) => {
//                     info!(&format!(
//                         "{:} -> General, response error: {:}",
//                         properties.definition.api_key, error
//                     ));
//                 }
//             },
//         }
//         true
//     }

//     fn changed(&mut self, ctx: &Context<Self>) -> bool {
//         ctx.link().send_message(super::message::Message::LoadPrices);
//         false
//     }
// }
