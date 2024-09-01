//! Current exchange trading rules and symbol information.
//!
//! - If any symbol provided in either symbol or symbols do not exist, the
//!   endpoint will throw an error.
//! - All parameters are optional.
//! - permissions can support single or multiple values (e.g. SPOT,
//!   ["MARGIN","LEVERAGED"])
//! - If permissions parameter not provided, the default values will be
//!   ["SPOT","MARGIN","LEVERAGED"].
//! - To display all permissions you need to specify them explicitly. (e.g.
//!   SPOT, MARGIN,...)
//!
//! Examples of Symbol Permissions Interpretation from the Response:
//! - [["A","B"]] means you may place an order if your account has either
//!   permission "A" or permission "B".
//! - [["A"],["B"]] means you can place an order if your account has permission
//!   "A" and permission "B".
//! - [["A"],["B","C"]] means you can place an order if your account has
//!   permission "A" and permission "B" or permission "C". (Inclusive or is applied here, not exclusive or, so your account may have both permission "B" and permission "C".)
//!
//! Weight(IP): 10

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderType, RateLimit},
    rest::endpoint,
};

endpoint!(
    ExchangeInfoEndpoint,
    "/api/v3/exchangeInfo",
    Method::GET,
    ExchangeInfoParams,
    ExchangeInfoResponse
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoParams {
    symbol: Option<String>,
    #[serde(serialize_with = "crate::rest::serialize_option_vec")]
    symbols: Option<Vec<String>>,
    permissions: Option<String>,
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

    pub fn symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn permissions(mut self, permissions: &str) -> Self {
        self.permissions = Some(permissions.to_string());
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_asset_precision: u64,
    pub base_commission_precision: u64,
    pub quote_commission_precision: u64,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub oto_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
}
