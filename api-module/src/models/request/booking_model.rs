use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BookingModel {
    pub id: Option<u32>,
    pub firstname: String,
    pub lastname: String,
    pub totalprice: u32,
    pub depositpaid: bool,
    pub bookingdates: BookingDates,
    pub additionalneeds: String,
}

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BookingDates {
    pub checkin: String,
    pub checkout: String,
}