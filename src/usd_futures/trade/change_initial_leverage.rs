use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/leverage",
    Method::POST,
    SecurityType::Trade,
    ChangeInitialLeverageEndpoint,
    ChangeInitialLeverageParams,
    ChangeInitialLeverageResponse
);

/// Change user's initial leverage of specific symbol market.
///
/// - Weight: 1
pub struct ChangeInitialLeverageEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ChangeInitialLeverageEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeInitialLeverageParams {
    symbol: String,
    leverage: i64,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ChangeInitialLeverageParams {
    pub fn new(symbol: &str, leverage: i64) -> Self {
        Self {
            symbol: symbol.to_owned(),
            leverage,
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
pub struct ChangeInitialLeverageResponse {
    pub leverage: i64,
    pub max_notional_value: String,
    pub symbol: String,
}
