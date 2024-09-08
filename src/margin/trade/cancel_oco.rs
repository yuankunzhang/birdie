use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::MarginOcoOrder;

impl Endpoint for CancelOcoEndpoint<'_> {
    type Response = CancelOcoResponse;
    type Params = CancelOcoParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/sapi/v1/margin/orderList"
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::Trade
    }
}

impl Params for CancelOcoParams {}

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

pub type CancelOcoResponse = MarginOcoOrder;
