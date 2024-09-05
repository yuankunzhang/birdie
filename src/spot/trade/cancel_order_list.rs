use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::OrderListResult;

impl Endpoint for CancelOrderListEndpoint<'_> {
    type Response = CancelOrderListResponse;
    type Params = CancelOrderListParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/api/v3/orderList"
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::Trade
    }
}

impl Params for CancelOrderListParams {}

/// Cancel an entire order list.
///
/// Additional notes:
///
/// - Canceling an individual order in an order list will cancel the entire
///   order list.
/// - If both `order_list_id` and `list_client_order_id` are sent,
///   `order_list_id` takes precedence.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct CancelOrderListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelOrderListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderListParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelOrderListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_list_id: None,
            list_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
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

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CancelOrderListResponse = OrderListResult;
