use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/account",
    Method::DELETE,
    SecurityType::Trade,
    DisableIsolatedMarginAccountEndpoint,
    DisableIsolatedMarginAccountParams,
    DisableIsolatedMarginAccountResponse
);

/// Disable isolated margin account for a specific symbol. Each trading pair can
/// only be deactivated once every 24 hours.
///
/// - Weight: 300
pub struct DisableIsolatedMarginAccountEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> DisableIsolatedMarginAccountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableIsolatedMarginAccountParams {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl DisableIsolatedMarginAccountParams {
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
pub struct DisableIsolatedMarginAccountResponse {
    pub success: bool,
    pub symbol: String,
}
