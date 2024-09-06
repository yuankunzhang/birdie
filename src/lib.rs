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
use hmac::{Hmac, Mac};
use rest_api::{RestApiClient, RestApiError};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use thiserror::Error;
use web_socket_api::WebSocketApiClient;

#[derive(Debug, Error)]
pub enum BirdieError {
    #[error("RestError: {0}")]
    RestApi(#[from] RestApiError),
}

pub struct Birdie {
    fix_api: FixApiClient,
    rest_api: RestApiClient,
    web_socket_api: WebSocketApiClient,
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, BirdieError> {
        let fix_api = FixApiClient::new();
        let rest_api = RestApiClient::new(base_url, api_key, secret_key)?;
        let web_socket_api =
            WebSocketApiClient::new("wss://stream.binance.com:9443/ws", api_key, secret_key);
        Ok(Self {
            fix_api,
            rest_api,
            web_socket_api,
        })
    }

    pub fn fix_api(&self) -> &FixApiClient {
        &self.fix_api
    }

    pub fn rest_api(&self) -> &RestApiClient {
        &self.rest_api
    }

    pub fn web_socket_api(&self) -> &WebSocketApiClient {
        &self.web_socket_api
    }
}

pub trait Params: Sized + Send + Serialize {
    fn as_query(&self) -> Result<String, RestApiError> {
        Ok(serde_qs::to_string(self)?)
    }
}

pub trait Response: Sized + for<'de> Deserialize<'de> {}

pub(crate) fn compute_signature(key: &str, data: &str) -> Result<String, RestApiError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())?;
    mac.update(data.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature() {
        let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let data = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let sig = compute_signature(key, data).unwrap();
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }
}
