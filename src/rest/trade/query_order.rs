use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderSide, OrderType, PreventionMode, TimeInForce},
    rest::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/order",
    Method::GET,
    SecurityType::UserData,
    QueryOrderEndpoint,
    QueryOrderParams,
    QueryOrderResponse
);

/// Check an order's status.
///
/// - Weight: 4
/// - Data Source: Memory => Database
pub struct QueryOrderEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> QueryOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOrderParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResponse {
    /// Example: `"LTCBTC"`
    pub symbol: String,
    /// Example: `1`
    pub order_id: i64,
    /// Example: `-1`
    pub order_list_id: i64,
    /// Example: `"myOrder1"`
    pub client_order_id: String,
    /// Example: `"0.1"`
    pub price: String,
    /// Example: `"1.0"`
    pub orig_qty: String,
    /// Example: `"0.0"`
    pub executed_qty: String,
    /// Example: `"0.0"`
    pub cummulative_quote_qty: String,
    /// Example: `"NEW"`
    pub status: String,
    /// Example: `"GTC"`
    pub time_in_force: TimeInForce,
    /// Example: `"LIMIT"`
    pub r#type: OrderType,
    /// Example: `"BUY"`
    pub side: OrderSide,
    /// Example: `"0.0"`
    pub stop_price: String,
    /// Example: `"0.0"`
    pub iceberg_qty: String,
    /// Example: `1499827319559`
    pub time: i64,
    /// Example: `1499827319559`
    pub update_time: i64,
    pub is_working: bool,
    /// Example: `1499827319559`
    pub working_time: i64,
    /// Example: `"0.00000000"`
    pub orig_quote_order_qty: String,
    /// Example: `"NONE"`
    pub self_trade_prevention_mode: PreventionMode,
}
