use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::OrderDetail;

impl Endpoint for QueryCurrentOpenOrderEndpoint<'_> {
    type Response = QueryCurrentOpenOrderResponse;
    type Params = QueryCurrentOpenOrderParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/fapi/v1/openOrder"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryCurrentOpenOrderParams {}

/// Query open order.
///
/// - Weight: 1
pub struct QueryCurrentOpenOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCurrentOpenOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCurrentOpenOrderParams {
    symbol: String,
    order_id: Option<i64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCurrentOpenOrderParams {
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryCurrentOpenOrderResponse = OrderDetail;
