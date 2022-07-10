use gloo_console::info;
use serde::{de::DeserializeOwned, Serialize};

use super::error::Error;

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    match response {
        Ok(data) => {
            let data: Result<T, _> = data.json::<T>().await;

            match data {
                Ok(result) => Ok(result),
                Err(error) => Err(Error {
                    msg: error.to_string(),
                }),
            }
        }
        Err(error) => Err(Error {
            msg: error.to_string(),
        }),
    }
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}
