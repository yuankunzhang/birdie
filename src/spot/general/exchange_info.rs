use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderType, RateLimit},
    filters::ExchangeFilter,
    rest_api::endpoint,
    web_socket_api::web_socket,
};

endpoint!(
    "/api/v3/exchangeInfo",
    Method::GET,
    ExchangeInfoEndpoint,
    ExchangeInfoParams,
    ExchangeInfoResponse
);

/// Current exchange trading rules and symbol information.
///
/// - Weight: 20
/// - Data Source: Memory
pub struct ExchangeInfoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ExchangeInfoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

/// ## Notes on parameters
///
/// - If the value provided to `symbol` or `symbols` do not exist, the endpoint
///   will throw [`BinanceErrorCode::BadSymbol`][`crate::errors::BinanceErrorCode::BadSymbol`]
///   error.
/// - If permissions parameter not provided, the default values will be
///   `["SPOT","MARGIN","LEVERAGED"]`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(serialize_with = "crate::rest_api::serialize_option_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(serialize_with = "crate::rest_api::serialize_option_vec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
}

impl Default for ExchangeInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeInfoParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            permissions: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn symbols(mut self, symbols: &[&str]) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn permissions(mut self, permissions: &[&str]) -> Self {
        self.permissions = Some(permissions.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub timezone: String,
    pub server_time: i64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: i64,
    pub quote_asset: String,
    pub quote_asset_precision: i64,
    pub base_commission_precision: i64,
    pub quote_commission_precision: i64,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub oto_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
}

web_socket!(
    "exchangeInfo",
    ExchangeInfoWebSocket,
    ExchangeInfoParams,
    ExchangeInfoResponse
);

pub struct ExchangeInfoWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> ExchangeInfoWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
