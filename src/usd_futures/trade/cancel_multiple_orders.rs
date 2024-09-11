use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

use super::CancelOrderDetail;

endpoint!(
    "/fapi/v1/batchOrders",
    Method::DELETE,
    SecurityType::Trade,
    CancelMultipleOrdersEndpoint,
    CancelMultipleOrdersParams,
    CancelMultipleOrdersResponse
);

/// Cancel Multiple Orders.
///
/// - Weight: 1
pub struct CancelMultipleOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelMultipleOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelMultipleOrdersParams {
    symbol: String,
    order_id_list: Option<Vec<i64>>,
    orig_client_order_id_list: Option<Vec<String>>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelMultipleOrdersParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id_list: None,
            orig_client_order_id_list: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_id_list(mut self, order_id_list: Vec<i64>) -> Self {
        self.order_id_list = Some(order_id_list);
        self
    }

    pub fn orig_client_order_id_list(mut self, orig_client_order_id_list: Vec<String>) -> Self {
        self.orig_client_order_id_list = Some(orig_client_order_id_list);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CancelMultipleOrdersResponse {
    Success(Vec<CancelOrderDetail>),
    Failure(BinanceError),
}
