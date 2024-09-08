use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/exchange-small-liability",
    Method::POST,
    SecurityType::Margin,
    SmallLiabilityExchangeEndpoint,
    SmallLiabilityExchangeParams,
    SmallLiabilityExchangeResponse
);

/// Small Liability Exchange
///
/// - Weight: 3000
pub struct SmallLiabilityExchangeEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> SmallLiabilityExchangeEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallLiabilityExchangeParams {
    asset_names: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl SmallLiabilityExchangeParams {
    pub fn new(asset_names: &str) -> Self {
        Self {
            asset_names: asset_names.to_owned(),
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
pub struct SmallLiabilityExchangeResponse {}
