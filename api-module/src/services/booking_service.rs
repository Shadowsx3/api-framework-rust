use crate::base::service_base::ServiceBase;
use crate::models::request::booking_model::BookingModel;
use crate::models::response::response::JsResponse;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use reqwest::Method;

#[napi]
pub struct BookingService {
  inner: ServiceBase,
}

#[napi]
impl BookingService {
  #[napi(constructor)]
  pub fn new() -> Self {
    BookingService {
      inner: ServiceBase::new("booking"),
    }
  }

  #[napi]
  pub async fn get_booking(&self, id: String) -> Result<JsResponse> {
    let url = format!("{}/{}", self.inner.url, id);
    let response = self
      .inner
      .request(Method::GET, &url, None::<()>)
      .await
      .unwrap();
    Ok(response)
  }

  #[napi]
  pub async fn add_booking(&self, booking: BookingModel) -> Result<JsResponse> {
    let response = self
      .inner
      .request(Method::POST, &self.inner.url, Some(&booking))
      .await
      .unwrap();
    Ok(response)
  }
}
