use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{futures::PositionSide, SecurityType},
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v3/positionRisk",
    Method::GET,
    SecurityType::UserData,
    PositionInformationV3Endpoint,
    PositionInformationV3Params,
    PositionInformationV3Response
);

/// Get current position information(only symbol that has position or open orders
/// will be returned).
///
/// - Weight: 5
pub struct PositionInformationV3Endpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> PositionInformationV3Endpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformationV3Params {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl PositionInformationV3Params {
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
pub enum PositionInformationV3Response {
    Item(PositionInformationV3),
    Vec(Vec<PositionInformationV3>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformationV3 {
    pub symbol: String,
    pub position_side: PositionSide,
    pub position_amt: String,
    pub entry_price: String,
    pub break_even_price: String,
    pub mark_price: String,
    pub un_realized_profit: String,
    pub liquidation_price: String,
    pub isolated_margin: String,
    pub notional: String,
    pub margin_asset: String,
    pub isolated_wallet: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub adl: i64,
    pub bid_notional: String,
    pub ask_notional: String,
    pub update_time: i64,
}
