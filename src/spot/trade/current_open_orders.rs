use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::SecurityType,
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::OrderDetail;

impl Endpoint for CurrentOpenOrdersEndpoint<'_> {
    type Response = CurrentOpenOrdersResponse;
    type Params = CurrentOpenOrdersParams;

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

impl Params for CurrentOpenOrdersParams {}

/// Get all open orders on a symbol. Careful when accessing this with no symbol.
///
/// - Weight:
///     - 6 for a single symbol;
///     - 80 when the symbol parameter is not omitted.
/// - Data Source: Memory => Database
pub struct CurrentOpenOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CurrentOpenOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentOpenOrdersParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CurrentOpenOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type CurrentOpenOrdersResponse = Vec<OrderDetail>;
