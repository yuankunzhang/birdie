use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::{endpoint, SecurityType};

endpoint!(
    "/api/v3/myTrades",
    Method::GET,
    SecurityType::UserData,
    AccountTradeListEndpoint,
    AccountTradeListParams,
    AccountTradeListResponse
);

/// Get trades for a specific account and symbol.
///
/// - Weight: 20
/// - Data Source: Memory => Database
pub struct AccountTradeListEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> AccountTradeListEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl AccountTradeListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    /// This can only be used in combination with `symbol`.
    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// TradeId to fetch from. Default gets most recent trades.
    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /// Default 500; max 1000.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type AccountTradeListResponse = Vec<MyTrade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTrade {
    /// Example: `"BNBBTC"`
    pub symbol: String,
    /// Example: `28457`
    pub id: i64,
    /// Example: `100234`
    pub order_id: i64,
    /// Example: `-1`
    pub order_list_id: i64,
    /// Example: `"4.00000100"`
    pub price: String,
    /// Example: `"12.00000000"`
    pub qty: String,
    /// Example: `"48.000012"`
    pub quote_qty: String,
    /// Example: `"10.10000000"`
    pub commission: String,
    /// Example: `"BNB"`
    pub commission_asset: String,
    /// Example: `1499865549590`
    pub time: i64,
    /// Example: `False`
    pub is_buyer: bool,
    /// Example: `False`
    pub is_maker: bool,
    pub is_best_match: bool,
}
