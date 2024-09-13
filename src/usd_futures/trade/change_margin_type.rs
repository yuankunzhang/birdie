use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

endpoint!(
    "/fapi/v1/marginType",
    Method::POST,
    SecurityType::Trade,
    ChangeMarginTypeEndpoint,
    ChangeMarginTypeParams,
    ChangeMarginTypeResponse
);

/// Change symbol level margin type.
///
/// - Weight: 1
pub struct ChangeMarginTypeEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ChangeMarginTypeEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeParams {
    symbol: String,
    margin_type: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ChangeMarginTypeParams {
    pub fn new(symbol: &str, margin_type: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            margin_type: margin_type.to_owned(),
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
#[serde(untagged)]
pub enum ChangeMarginTypeResponse {
    Success,
    Failure(BinanceError),
}
