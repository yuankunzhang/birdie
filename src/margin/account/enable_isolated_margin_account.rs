use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/account",
    Method::POST,
    SecurityType::Trade,
    EnableIsolatedMarginAccountEndpoint,
    EnableIsolatedMarginAccountParams,
    EnableIsolatedMarginAccountResponse
);

/// Enable isolated margin account for a specific symbol(Only supports
/// activation of previously disabled accounts).
///
/// - Weight: 300
pub struct EnableIsolatedMarginAccountEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> EnableIsolatedMarginAccountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableIsolatedMarginAccountParams {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl EnableIsolatedMarginAccountParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableIsolatedMarginAccountResponse {
    pub success: bool,
    pub symbol: String,
}
