#![deny(clippy::all)]

extern crate napi_derive;

mod base;
mod models;
mod services;

pub use services::booking_service::BookingService;
pub use models::request::booking_model::BookingModel;
