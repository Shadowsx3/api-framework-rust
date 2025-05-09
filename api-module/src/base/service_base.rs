use crate::base::api_client::ApiClient;
use crate::models::response::response::JsResponse;
use napi::bindgen_prelude::BigInt;
use napi::Error;
use reqwest::{Client, Method};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

pub struct ServiceBase {
  pub url: String,
  pub client: Arc<Client>,
}

impl ServiceBase {
  pub fn new(endpoint_path: &str) -> Self {
    let base_url =
      std::env::var("BASEURL").unwrap_or("https://restful-booker.herokuapp.com".to_owned());
    let url = format!(
      "{}/{}",
      base_url.trim_end_matches('/'),
      endpoint_path.trim_start_matches('/')
    );

    Self {
      url,
      client: ApiClient::get_instance().client(),
    }
  }

  pub async fn request(
    &self,
    method: Method,
    url: &str,
    body: Option<impl Serialize>,
  ) -> Result<JsResponse, Error> {
    let start = Instant::now();
    let mut req = self.client.request(method, url);

    if let Some(b) = body {
      req = req.json(&b);
    }

    let resp = req.send().await.map_err(|err| {
      eprintln!("Request failed: {}", err);
      Error::new(
        napi::Status::GenericFailure,
        format!("Request failed: {}", err),
      )
    })?;

    let elapsed = start.elapsed().as_millis();

    let status = resp.status().as_u16();
    let headers = convert_headers(resp.headers());
    let text = resp.text().await.map_err(|err| {
      eprintln!("Failed to read response text: {}", err);
      Error::new(
        napi::Status::GenericFailure,
        format!("Failed to read response text: {}", err),
      )
    })?;

    let data: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
      eprintln!("Failed to parse JSON: {}", text);
      Error::new(
        napi::Status::GenericFailure,
        format!("Failed to parse JSON: {}", err),
      )
    })?;

    Ok(JsResponse {
      data,
      status,
      headers,
      response_time: BigInt::from(elapsed),
    })
  }
}

fn convert_headers(headers: &reqwest::header::HeaderMap) -> HashMap<String, String> {
  headers
    .iter()
    .map(|(k, v)| {
      let key = k.to_string();
      let val = v.to_str().unwrap_or("").to_string();
      (key, val)
    })
    .collect()
}
