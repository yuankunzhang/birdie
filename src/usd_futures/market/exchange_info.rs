use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderType, RateLimit},
    filters::ExchangeFilter,
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/exchangeInfo",
    Method::GET,
    ExchangeInfoEndpoint,
    ExchangeInfoParams,
    ExchangeInfoResponse
);

/// Current exchange trading rules and symbol information.
///
/// - Weight: 1
pub struct ExchangeInfoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ExchangeInfoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoParams {}

impl Default for ExchangeInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeInfoParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: i64,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Symbol>,
    pub timezone: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: i64,
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
