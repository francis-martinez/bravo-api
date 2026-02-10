#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod models;

use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub async fn ping() -> Result<JsValue, JsValue> {
    let config = apis::configuration::Configuration::default();

    match apis::default_api::get_ping(&config).await {
        Ok(ping) => Ok(to_value(&ping).unwrap()),
        Err(e) => Err(JsValue::from_str(&format!("Error fetching ping: {}", e))),
    }
}
