use chrono::Local;
use gloo_console::info;
use std::vec;
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

use crate::agents::EventBus;
use crate::common::*;
use crate::models::Crypto;

use load_dotenv::load_dotenv;
load_dotenv!();

const API_ROOT: &str = env!("API_URL");

// !!!!!!!!!!!!!!!! TAKO JE TO !!!!!!!!!!!!!!!!
//  // url for request
//  let url_request = format!("{:}{:}", env!("API_URL"), "/definitions");
//  info!(&format!("Definitions -> Loading data: {:?}", url_request));

//  ctx.link().send_future(async move {
//      let url_request = format!("https://api.coingecko.com/api/v3/coins/{:}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false","api_key");

//      info!(&format!(
//          "General component: {:} -> Loading data: {:?}",
//          "api_key", url_request
//      ));

//      let response = request_get::<Vec<Crypto>>(url_request).await;

//      Message::DefinitionsLoaded(response)
//  });

#[derive(Properties, PartialEq)]
pub struct AnalyticsProperties {
    pub last_updated: Option<chrono::DateTime<Local>>,
}

#[function_component(Analytics)]
pub fn analytics(props: &AnalyticsProperties) -> Html {
    info!("~Analytics component~");

    let crypto_definitions = use_async(async move {
        let url = format!("{}{}", API_ROOT, "/definitions");
        info!(&format!("Analytics component: Loading definitions {}", url));

        request_get::<Vec<Crypto>>(url).await
    });

    // let crypto_definitions = use_async_with_options(
    //     async move {
    //         let url = format!("{}{}", API_ROOT, "/definitions");
    //         info!(&format!("Analytics component: Loading definitions {}", url));

    //         request_get::<Vec<Crypto>>(url).await
    //     },
    //     UseAsyncOptions { auto: true },
    // );

    //crypto_definitions.run();

    // let interval = {
    //     let crypto_definitions = crypto_definitions.clone();
    //     Callback::from(move |_| {
    //         //let crypto_definitions = crypto_definitions.clone();
    //         crypto_definitions.run();
    //     })
    // };

    // use_effect(move || {
    //     let producer = EventBus::bridge(interval);

    //     || drop(producer)
    // });

    let crypto_html: Vec<Html>;

    if let Some(crypto_definitions) = &crypto_definitions.data {
        crypto_html = crypto_definitions
            .iter()
            .map(|crypto_definition| {
                html! {
                   <super::general::General definition={crypto_definition.clone()}/>
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

    if let Some(errors) = &crypto_definitions.error {
        info!(&format!(
            "Analytics component: Loading definitions error: {}",
            errors
        ));
    }

    html! {
        <div class={classes!("analytics-container")}>
            {crypto_html}
        </div>
    }
}

//let agent = EventBus::bridge(interval);

//{
// let crypto_definitions = crypto_definitions.clone();
// use_effect_with_deps(
//     move |_| {
//         crypto_definitions.run();
//         || ()
//     },
//     props.last_updated.clone(),
// );

//let counter = counter.clone();
// use_effect(move || {
//     // Make a call to DOM API after component is rendered
//     gloo_utils::document().set_title(&format!("You clicked {} times", *counter));

//     // Perform the cleanup
//     || gloo_utils::document().set_title("You clicked 0 times")
// });
//}
// let onclick = {
//     info!("~Analytics component~ OnClick");
//     let counter = counter.clone();
//     Callback::from(move |_| {
//         info!("~Analytics component~ OnClick counter");
//         counter.set(*counter + 1);
//     })
// };

// html! {
//     <button>{ format!("Increment to {}", *counter) }</button>
// }

// let current_page = use_state(|| None::<chrono::DateTime<Local>>);

// let crypto_definitions =
//     use_async(async move { request_get::<Vec<Crypto>>(String::from("/definitions")).await });

// {
//     let current_page = current_page.clone();
//     use_effect(move || {
//         // Reset to first page
//         current_page.set(None::<chrono::DateTime<Local>>);
//         || ()
//     });
// }

// {
//     let crypto_definitions = crypto_definitions.clone();
//     use_effect(move || {
//         crypto_definitions.run();
//         || ()
//     });
// }

// let callback = {
//     let current_page = current_page.clone();

//     Callback::from(move |_| {
//         current_page.set(Some(chrono::offset::Local::now()));
//     })
// };
