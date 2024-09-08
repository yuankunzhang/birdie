use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params, Response,
};

use super::MarginOrderDetail;

impl Endpoint for QueryOrderEndpoint<'_> {
    type Response = QueryOrderResponse;
    type Params = QueryOrderParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/sapi/v1/margin/order"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryOrderParams {}

/// Query Margin Account's Order.
///
/// - Weight: 10
pub struct QueryOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderParams {
    symbol: String,
    is_isolated: Option<String>,
    order_id: Option<i64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOrderParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            is_isolated: None,
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn is_isolated(mut self, is_isolated: &str) -> Self {
        self.is_isolated = Some(is_isolated.to_owned());
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

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryOrderResponse = MarginOrderDetail;

impl Response for QueryOrderResponse {}
