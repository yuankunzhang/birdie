use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::OrderDetail;

impl Endpoint for QueryCurrentAllOpenOrdersEndpoint<'_> {
    type Response = QueryCurrentAllOpenOrdersResponse;
    type Params = QueryCurrentAllOpenOrdersParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/fapi/v1/openOrders"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryCurrentAllOpenOrdersParams {}

/// Get all open orders on a symbol.
///
/// - Weight:
pub struct QueryCurrentAllOpenOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCurrentAllOpenOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCurrentAllOpenOrdersParams {
    symbol: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCurrentAllOpenOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryCurrentAllOpenOrdersResponse = Vec<OrderDetail>;
