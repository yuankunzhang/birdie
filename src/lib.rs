//! # Birdie - Binance Rust Development Kit
//!
//! Birdie is a third party Binance API client, allowing you to easily interact
//! with the Binance API using Rust.
//!
//! ## Read First
//!
//! - Always test your code.
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
//! - [`mod@rest_api`] - REST API client.
//! - [`mod@web_socket_api`] - Web Socket API client.
//! - [`web_socket_stream`] - Web Socket stream client.
//!
//! ## REST API Client
//!
//! To interact with the Binance REST API, create a REST API client first.
//!
//! ```rust
//! let base_url = "https://api.binance.com";
//! let api_key = "your_api_key";
//! let api_secret = "your_api_secret";
//! let client = birdie::rest_api(base_url, api_key, api_secret).unwrap();
//! ```
//!
//! Once you have the client, you can access the different categories of the
//! API and the different endpoints. For example, to access the account
//! information endpoint (note how the endpoint is accessed):
//!
//! ```no_run
//! use birdie::{rest_api::Endpoint, spot::account::AccountInformationParams};
//!
//! let params = AccountInformationParams::new().omit_zero_balances(true);
//! let resp = client.account().account_information().request(params).await;
//! assert!(resp.is_ok());
//! ```
//!
//! See the [`mod@rest_api`] module for more information.

pub mod enums;
pub mod errors;
pub mod filters;
pub mod fix_api;
pub mod rest_api;
pub mod web_socket;
pub mod web_socket_api;
pub mod web_socket_stream;

pub mod margin;
pub mod spot;

use base64::{engine::general_purpose::STANDARD as b64, Engine};
use ed25519_dalek::{ed25519::signature::SignerMut, pkcs8::DecodePrivateKey, SigningKey};
use hmac::{Hmac, Mac};
use rest_api::{RestApiClient, RestApiError};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub fn rest_api(
    base_url: &str,
    api_key: &str,
    secret_key: &str,
) -> Result<RestApiClient, RestApiError> {
    RestApiClient::new(base_url, api_key, secret_key)
}

pub fn web_socket_api(
    endpoint: &str,
    api_key: &str,
    secret_key: &str,
) -> web_socket_api::WebSocketApiClient {
    web_socket_api::WebSocketApiClient::new(endpoint, api_key, secret_key)
}

pub trait Params: Sized + Send + Serialize {
    fn as_query(&self) -> Result<String, RestApiError> {
        Ok(serde_qs::to_string(self)?)
    }
}

pub trait Response: Sized + for<'de> Deserialize<'de> {}

pub(crate) fn hmac_signature(key: &str, data: &str) -> Result<String, hmac::digest::InvalidLength> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())?;
    mac.update(data.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

pub(crate) fn ed25519_signature(
    key: &str,
    data: &str,
) -> Result<String, ed25519_dalek::pkcs8::Error> {
    let mut key = SigningKey::from_pkcs8_pem(key)?;
    let signature = key.sign(data.as_bytes());
    let encoded = b64.encode(signature.to_bytes());
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmac() {
        let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let data = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let sig = hmac_signature(key, data).unwrap();
        assert_eq!(
            sig,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }

    #[tokio::test]
    async fn ed25519() {
        let pem = r#"\
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIHWPzTvl8pIHsAbZtJv+/0kaXO611fs90IewpT1PEwFT
-----END PRIVATE KEY-----
"#;
        let data = "hello world";
        let signature = ed25519_signature(pem, data).unwrap();
        assert_eq!(signature, "e2jzxQfzCeKvlScapTXk1Jt7e1i6rIXLZ4UVWcJ7kykMSARX/rUV7a3za+LlFizFUORuUs/38zlVbxFnrcCLBw==");
    }
}
