use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    web_socket_api::web_socket,
    Params,
};

use super::OrderListResult;

impl Endpoint for QueryOrderListsEndpoint<'_> {
    type Response = QueryOrderListsResponse;
    type Params = QueryOrderListsParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/api/v3/orderList"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryOrderListsParams {}

/// Retrieves a specific order list based on provided optional parameters.
///
/// - Weight: 4
/// - Data Source: Database
pub struct QueryOrderListsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOrderListsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

/// Notes:
///
/// - Either `order_id` or `orig_client_order_id` must be sent.
/// - For some historical orders `cummulative_quote_qty` will be < 0, meaning
///   the data is not available at this time.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderListsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOrderListsParams {
    pub fn new() -> Self {
        Self {
            order_list_id: None,
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_list_id(mut self, order_list_id: i64) -> Self {
        self.order_list_id = Some(order_list_id);
        self
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

pub type QueryOrderListsResponse = OrderListResult;

web_socket!(
    "orderList.status",
    QueryOrderListsWebSocket,
    QueryOrderListsParams,
    QueryOrderListsResponse
);

pub struct QueryOrderListsWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> QueryOrderListsWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
