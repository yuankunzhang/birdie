use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

endpoint!(
    "/fapi/v1/positionSide/dual",
    Method::POST,
    SecurityType::Trade,
    ChangePositionModeEndpoint,
    ChangePositionModeParams,
    ChangePositionModeResponse
);

/// Change symbol level margin type.
///
/// - Weight: 1
pub struct ChangePositionModeEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ChangePositionModeEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeParams {
    dual_side_position: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ChangePositionModeParams {
    pub fn new(dual_side_position: &str) -> Self {
        Self {
            dual_side_position: dual_side_position.to_owned(),
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
pub enum ChangePositionModeResponse {
    Success,
    Failure(BinanceError),
}
