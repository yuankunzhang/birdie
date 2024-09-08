use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/manual-liquidation",
    Method::POST,
    SecurityType::Margin,
    MarginManualLiquidationEndpoint,
    MarginManualLiquidationParams,
    MarginManualLiquidationResponse
);

/// Margin Manual Liquidation.
///
/// - Weight: 3000
pub struct MarginManualLiquidationEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> MarginManualLiquidationEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginManualLiquidationParams {
    r#type: String,
    symbol: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl MarginManualLiquidationParams {
    pub fn new(r#type: &str) -> Self {
        Self {
            r#type: r#type.to_owned(),
            symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type MarginManualLiquidationResponse = Vec<MarginManualLiquidation>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginManualLiquidation {
    pub asset: String,
    pub interest: String,
    pub principal: String,
    pub liquidation_asset: String,
    pub liquidation_qty: String,
}
