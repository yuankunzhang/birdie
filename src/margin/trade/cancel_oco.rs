use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        ContingencyType, OrderSide, OrderStatus, OrderType, SecurityType, SelfTradePreventionMode,
        TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/orderList",
    Method::DELETE,
    SecurityType::Trade,
    CancelOcoEndpoint,
    CancelOcoParams,
    CancelOcoResponse
);

/// Cancel an entire Order List for a margin account.
///
/// - Weight: 1
pub struct CancelOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOcoParams {
    symbol: String,
    isolated: Option<String>,
    order_list_id: Option<i64>,
    list_client_order_id: Option<String>,
    new_client_order_id: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelOcoParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            isolated: None,
            order_list_id: None,
            list_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn isolated(mut self, isolated: &str) -> Self {
        self.isolated = Some(isolated.to_owned());
        self
    }

    pub fn order_list_id(mut self, order_list_id: i64) -> Self {
        self.order_list_id = Some(order_list_id);
        self
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelOcoResponse = OrderListResult;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResult {
    pub order_list_id: i64,
    pub contingency_type: ContingencyType,
    pub list_status_type: String,
    pub list_order_status: String,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    pub is_isolated: bool,
    #[serde(default)]
    pub orders: Vec<OrderListOrder>,
    #[serde(default)]
    pub order_reports: Vec<OrderListOrderReport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListOrder {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListOrderReport {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub iceberg_qty: Option<String>,
    pub stop_price: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}
