use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::MarginOcoOrder;

impl Endpoint for QueryOcoEndpoint<'_> {
    type Response = QueryOcoResponse;
    type Params = QueryOcoParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/sapi/v1/margin/order/orderList"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::UserData
    }
}

impl Params for QueryOcoParams {}

/// Retrieves a specific OCO based on provided optional parameters.
///
/// - Weight: 10
pub struct QueryOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOcoParams {
    symbol: Option<String>,
    is_isolated: Option<String>,
    order_list_id: Option<i64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryOcoParams {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
            order_list_id: None,
            orig_client_order_id: None,
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

    pub fn order_list_id(mut self, order_list_id: i64) -> Self {
        self.order_list_id = Some(order_list_id);
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

pub type QueryOcoResponse = Vec<MarginOcoOrder>;
