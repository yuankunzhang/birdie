//! # Birdie - Binance Rust Development Kit
//!
//! Birdie is a third party Binance API client, allowing you to easily interact
//! with the Binance API using Rust.
//!
//! ## Read First
//!
//! - This is a third part client, please refer to the official Binance API
//!   documentation for the most up-to-date information.
//! - Since the API specification doesn't specify number sizes or signedness,
//!   all integer fields are treated as `i64`, all floating point fields are
//!   treated as `f64`.
//!
//! ## Components
//!
//! Birdie is divided into several components, each representing a different
//! part of the Binance API:
//!
//! - [`fix_api`] - FIX API client (stub).
//! - [`rest_api`] - REST API client.
//!
//! To start using Birdie, you need to create a instance of the `Birdie`
//! struct. This struct contains all the components you need to interact with
//! the Binance API.
//!
//! ```rust
//! use birdie::Birdie;
//!
//! let base_url = "https://api.binance.com";
//! let api_key = "your_api_key";
//! let api_secret = "your_api_secret";
//! let birdie = Birdie::new(base_url, api_key, api_secret).unwrap();
//! ```
//!
//! Once you have a `Birdie` instance, you can access the different components
//! by calling the corresponding methods. For example, to access the REST API
//! client: `let rest = birdie.rest_api()`. Read the documentation for each
//! component to learn how to use them.

pub mod enums;
pub mod errors;
pub mod filters;
pub mod fix_api;
pub mod models;
pub mod rest_api;
pub mod web_socket_api;

pub mod spot;

use fix_api::FixApiClient;
use rest_api::{RestApiClient, RestApiError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BirdieError {
    #[error("RestError: {0}")]
    RestApi(#[from] RestApiError),
}

pub struct Birdie {
    fix_api: FixApiClient,
    rest_api: RestApiClient,
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, BirdieError> {
        let fix_api = FixApiClient::new();
        let rest_api = RestApiClient::new(base_url, api_key, secret_key)?;
        // let web_socket_api =
        //     WebSocketApiClient::new("wss://stream.binance.com:9443/ws", api_key, secret_key);
        Ok(Self { fix_api, rest_api })
    }

    pub fn fix_api(&self) -> &FixApiClient {
        &self.fix_api
    }

    pub fn rest_api(&self) -> &RestApiClient {
        &self.rest_api
    }
}
