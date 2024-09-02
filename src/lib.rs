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
pub mod models;
pub mod rest;
pub mod web_socket;

use rest::{RestClient, RestError};
use thiserror::Error;
use web_socket::WebSocketClient;

#[derive(Debug, Error)]
pub enum BirdieError {
    #[error("RestError: {0}")]
    Rest(#[from] RestError),
}

pub struct Birdie {
    rest: RestClient,
    web_socket: WebSocketClient,
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, BirdieError> {
        let rest = RestClient::new(base_url, api_key, secret_key)?;
        let web_socket = WebSocketClient::new();
        Ok(Self { rest, web_socket })
    }

    pub fn rest(&self) -> &RestClient {
        &self.rest
    }

    pub fn web_socket(&self) -> &WebSocketClient {
        &self.web_socket
    }
}
