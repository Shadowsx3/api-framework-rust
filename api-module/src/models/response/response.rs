use napi_derive::napi;
use std::collections::HashMap;
use napi::bindgen_prelude::BigInt;
use serde_json::Value;

#[napi(object)]
pub struct JsResponse {
    pub data: Value,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub response_time: BigInt,
}