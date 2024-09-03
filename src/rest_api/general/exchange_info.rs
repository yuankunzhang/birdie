use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::RateLimit, filters::ExchangeFilter, models::Symbol, rest_api::endpoint};

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
    client: &'r crate::rest_api::RestClient,
}

impl<'r> ExchangeInfoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestClient) -> Self {
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
    symbol: Option<String>,
    #[serde(serialize_with = "crate::rest_api::serialize_option_vec")]
    symbols: Option<Vec<String>>,
    #[serde(serialize_with = "crate::rest_api::serialize_option_vec")]
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
