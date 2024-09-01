use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderType, RateLimit},
    rest::{Endpoint, Params, Response, RestClient},
};

/// Current exchange trading rules and symbol information.
///
/// - If any symbol provided in either symbol or symbols do not exist, the
///   endpoint will throw an error.
/// - All parameters are optional.
/// - permissions can support single or multiple values (e.g. SPOT,
///   ["MARGIN","LEVERAGED"])
/// - If permissions parameter not provided, the default values will be
///   ["SPOT","MARGIN","LEVERAGED"].
/// - To display all permissions you need to specify them explicitly. (e.g.
///   SPOT, MARGIN,...)
///
/// Examples of Symbol Permissions Interpretation from the Response:
/// - [["A","B"]] means you may place an order if your account has either
///   permission "A" or permission "B".
/// - [["A"],["B"]] means you can place an order if your account has permission
///   "A" and permission "B".
/// - [["A"],["B","C"]] means you can place an order if your account has
///   permission "A" and permission "B" or permission "C". (Inclusive or is applied here, not exclusive or, so your account may have both permission "B" and permission "C".)
///
/// Weight(IP): 10
pub struct ExchangeInfoEndpoint<'r> {
    client: &'r RestClient,
}

impl<'r> ExchangeInfoEndpoint<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Endpoint for ExchangeInfoEndpoint<'_> {
    type Params = ExchangeInfoParams;
    type Response = ExchangeInfoResponse;

    fn client(&self) -> &RestClient {
        self.client
    }

    fn path(&self) -> &str {
        "api/v3/exchangeInfo"
    }

    fn method(&self) -> Method {
        Method::GET
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoParams {}

impl Params for ExchangeInfoParams {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

impl Response for ExchangeInfoResponse {}

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
