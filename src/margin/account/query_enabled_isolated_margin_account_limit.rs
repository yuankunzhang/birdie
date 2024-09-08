use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/accountLimit",
    Method::GET,
    SecurityType::UserData,
    QueryEnabledIsolatedMarginAccountLimitEndpoint,
    QueryEnabledIsolatedMarginAccountLimitParams,
    QueryEnabledIsolatedMarginAccountLimitResponse
);

/// Query enabled isolated margin account limit.
///
/// - Weight: 1
pub struct QueryEnabledIsolatedMarginAccountLimitEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryEnabledIsolatedMarginAccountLimitEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryEnabledIsolatedMarginAccountLimitParams {
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryEnabledIsolatedMarginAccountLimitParams {
    pub fn new() -> Self {
        Self {
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
pub struct QueryEnabledIsolatedMarginAccountLimitResponse {
    pub enabled_account: i64,
    pub max_account: i64,
}
