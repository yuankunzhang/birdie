use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/orderAmendment",
    Method::GET,
    SecurityType::Trade,
    GetOrderModifyHistoryEndpoint,
    GetOrderModifyHistoryParams,
    GetOrderModifyHistoryResponse
);

/// Get order modification history.
///
/// - Weight: 1
pub struct GetOrderModifyHistoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetOrderModifyHistoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderModifyHistoryParams {
    symbol: String,
    order_id: Option<i64>,
    orig_client_order_id: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetOrderModifyHistoryParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
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

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type GetOrderModifyHistoryResponse = Vec<OrderModifyRecord>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderModifyRecord {
    pub amendment_id: i64,
    pub symbol: String,
    pub pair: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    pub price: PriceAmendment,
    pub orig_qty: QuantityAmendment,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAmendment {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityAmendment {
    pub before: String,
    pub after: String,
}
