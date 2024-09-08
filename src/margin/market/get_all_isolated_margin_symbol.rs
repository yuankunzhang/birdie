use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/allPairs",
    Method::GET,
    SecurityType::MarketData,
    GetAllIsolatedMarginSymbolEndpoint,
    GetAllIsolatedMarginSymbolParams,
    GetAllIsolatedMarginSymbolResponse
);

/// Get All Isolated Margin Symbol.
///
/// - Weight: 10
pub struct GetAllIsolatedMarginSymbolEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetAllIsolatedMarginSymbolEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllIsolatedMarginSymbolParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetAllIsolatedMarginSymbolParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type GetAllIsolatedMarginSymbolResponse = Vec<IsolatedMarginSymbol>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginSymbol {
    pub base: String,
    pub is_buy_allowed: bool,
    pub is_margin_trade: bool,
    pub is_sell_allowed: bool,
    pub quote: String,
    pub symbol: String,
}
