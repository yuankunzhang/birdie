//! # Birdie - Binance Rust Development Kit
//!
//! Birdie is a third party Binance API client, allowing you to easily interact
//! with the Binance API using Rust.
//!
//! ## Important Notes
//!
//! - The API specification is subject to change, please refer to the official
//!   Binance API documentation for the most up-to-date information.
//! - Since the API specification doesn't specify number sizes or signedness,
//!   all integer fields are treated as `i64`, all floating point fields are
//!   treated as `f64`.

pub mod enums;
pub mod errors;
pub mod filters;
pub mod fix;
pub mod models;
pub mod rest_api;
pub mod web_socket;

pub mod account;
pub mod general;
pub mod market;
pub mod trade;

use fix::FixClient;
use rest_api::{RestApiClient, RestApiError};
use thiserror::Error;
use web_socket::WebSocketClient;

#[derive(Debug, Error)]
pub enum BirdieError {
    #[error("RestError: {0}")]
    RestApi(#[from] RestApiError),
}

pub struct Birdie {
    fix_api: FixClient,
    rest_api: RestApiClient,
    web_socket: WebSocketClient,
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, BirdieError> {
        let fix_api = FixClient::new();
        let rest_api = RestApiClient::new(base_url, api_key, secret_key)?;
        let web_socket = WebSocketClient::new();
        Ok(Self {
            fix_api,
            rest_api,
            web_socket,
        })
    }

    pub fn fix_api(&self) -> &FixClient {
        &self.fix_api
    }

    pub fn rest_api(&self) -> &RestApiClient {
        &self.rest_api
    }

    pub fn web_socket(&self) -> &WebSocketClient {
        &self.web_socket
    }
}
