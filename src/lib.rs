//! Birdie - Binance Rust Development Kit
//!
//! ## API Key Setup
//!
//! - Some endpoints will require an API Key. Please refer to this page regarding
//!   API key creation.
//! - Once API key is created, it is recommended to set IP restrictions on the
//!   key for security reasons.
//! - Never share your API key/secret key to ANYONE.
//!
//! ## API Key Restrictions
//!
//! - After creating the API key, the default restrictions is Enable Reading.
//! - To enable withdrawals via the API, the API key restriction needs to be
//!   modified through the Binance UI.
//!
//! ## Enabling Accounts
//!
//! - **Spot Account**. A SPOT account is provided by default upon creation of a
//!   Binance Account.
//! - **Margin Account**. To enable a MARGIN account for Margin Trading, please
//!   refer to the
//!   [Margin Trading Guide](https://www.binance.vision/tutorials/binance-margin-trading-guide).
//! - **Spot Testnet**. Users can use the SPOT Testnet to practice SPOT trading.
//!   Currently, this is only available via the API. Please refer to the
//!   [SPOT Testnet page](https://testnet.binance.vision/) for more information
//!   and how to set up the Testnet API key.
//!
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

use rest::{RestClient, RestError};
use thiserror::Error;

pub struct Birdie {
    rest: RestClient,
}

#[derive(Debug, Error)]
pub enum BirdieError {
    #[error("RestError: {0}")]
    Rest(#[from] RestError),
}

impl Birdie {
    pub fn new(base_url: &str, api_key: &str, secret_key: &str) -> Result<Self, BirdieError> {
        let rest = RestClient::new(base_url, api_key, secret_key)?;
        Ok(Self { rest })
    }

    pub fn rest(&self) -> &RestClient {
        &self.rest
    }
}
