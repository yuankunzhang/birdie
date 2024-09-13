use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{futures::PositionSide, SecurityType},
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/positionMargin",
    Method::POST,
    SecurityType::Trade,
    ModifyIsolatedPositionMarginEndpoint,
    ModifyIsolatedPositionMarginParams,
    ModifyIsolatedPositionMarginResponse
);

/// Modify Isolated Position Margin.
///
/// - Weight: 1
pub struct ModifyIsolatedPositionMarginEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ModifyIsolatedPositionMarginEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIsolatedPositionMarginParams {
    symbol: String,
    position_side: Option<PositionSide>,
    amount: Option<f64>,
    r#type: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ModifyIsolatedPositionMarginParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            position_side: None,
            amount: None,
            r#type: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn r#type(mut self, r#type: i64) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIsolatedPositionMarginResponse {
    pub amount: f64,
    pub code: i64,
    pub msg: String,
    pub r#type: i64,
}
