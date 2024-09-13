use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{futures::PositionSide, SecurityType},
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v2/positionRisk",
    Method::GET,
    SecurityType::UserData,
    PositionInformationV2Endpoint,
    PositionInformationV2Params,
    PositionInformationV2Response
);

/// Get current position information.
///
/// - Weight: 5
pub struct PositionInformationV2Endpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> PositionInformationV2Endpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformationV2Params {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl PositionInformationV2Params {
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
#[serde(untagged)]
pub enum PositionInformationV2Response {
    Item(PositionInformationV2),
    Vec(Vec<PositionInformationV2>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformationV2 {
    pub entry_price: String,
    pub break_even_price: String,
    pub margin_type: String,
    pub is_auto_add_margin: String,
    pub isolated_margin: String,
    pub leverage: String,
    pub liquidation_price: String,
    pub mark_price: String,
    pub max_notional_value: String,
    pub position_amt: String,
    pub notional: String,
    pub isolated_wallet: String,
    pub symbol: String,
    pub un_realized_profit: String,
    pub position_side: PositionSide,
    pub update_time: i64,
}
