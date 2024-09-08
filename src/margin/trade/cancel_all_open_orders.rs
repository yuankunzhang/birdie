use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{ContingencyType, SecurityType},
    rest_api::endpoint,
};

use super::{CancelOrderResult, OrderListOrder, OrderListOrderReport};

endpoint!(
    "/sapi/v1/margin/openOrders",
    Method::DELETE,
    SecurityType::Trade,
    CancelAllOpenOrdersEndpoint,
    CancelAllOpenOrdersParams,
    CancelAllOpenOrdersResponse
);

/// Cancels all active orders on a symbol for margin account.
///
/// This includes OCO orders.
///
/// - Weight: 100
pub struct CancelAllOpenOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelAllOpenOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersParams {
    symbol: String,
    isolated: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelAllOpenOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            isolated: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn isolated(mut self, isolated: &str) -> Self {
        self.isolated = Some(isolated.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelAllOpenOrdersResponse = Vec<CancelAllOpenOrdersResult>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CancelAllOpenOrdersResult {
    CancelOrderResult(CancelOrderResult),
    CancelOrderListResult(CancelOrderListResult),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderListResult {
    pub order_list_id: i64,
    pub contingency_type: ContingencyType,
    pub list_status_type: String,
    pub list_order_status: String,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    pub orders: Vec<OrderListOrder>,
    pub order_reports: Vec<OrderListOrderReport>,
}
