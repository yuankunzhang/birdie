use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

use super::{NewOrderDetail, NewOrderInput};

endpoint!(
    "/fapi/v1/order",
    Method::POST,
    SecurityType::Trade,
    PlaceMultipleOrdersEndpoint,
    PlaceMultipleOrdersParams,
    PlaceMultipleOrdersResponse
);

/// Send in a new order.
///
/// - Weight:
pub struct PlaceMultipleOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> PlaceMultipleOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceMultipleOrdersParams {
    #[serde(flatten)]
    input: Vec<NewOrderInput>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl PlaceMultipleOrdersParams {
    pub fn new(input: Vec<NewOrderInput>) -> Self {
        Self {
            input,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PlaceMultipleOrdersResponse {
    Success(NewOrderDetail),
    Failure(BinanceError),
}
