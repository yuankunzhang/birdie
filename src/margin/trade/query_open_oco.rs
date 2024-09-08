use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::OrderListResult;

impl Endpoint for QueryOpenOcoEndpoint<'_> {
    type Response = QueryOpenOcoResponse;
    type Params = QueryOpenOcoParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/sapi/v1/margin/openOrderList"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryOpenOcoParams {}

/// Query Margin Account's Open OCO.
///
/// - Weight: 10
pub struct QueryOpenOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOpenOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpenOcoParams {
    symbol: Option<String>,
    is_isolated: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOpenOcoParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn is_isolated(mut self, is_isolated: &str) -> Self {
        self.is_isolated = Some(is_isolated.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryOpenOcoResponse = Vec<OrderListResult>;
