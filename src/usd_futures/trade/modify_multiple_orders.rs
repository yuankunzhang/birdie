use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, errors::BinanceError, rest_api::endpoint};

use super::{ModifyOrderDetail, ModifyOrderInput};

endpoint!(
    "/fapi/v1/batchOrders",
    Method::PUT,
    SecurityType::Trade,
    ModifyMultipleOrdersEndpoint,
    ModifyMultipleOrdersParams,
    ModifyMultipleOrdersResponse
);

/// Order modify function, currently only LIMIT order modification is supported,
/// modified orders will be reordered in the match queue.
///
/// - Weight:
pub struct ModifyMultipleOrdersEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ModifyMultipleOrdersEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyMultipleOrdersParams {
    #[serde(flatten)]
    input: ModifyOrderInput,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ModifyMultipleOrdersParams {
    pub fn new(input: ModifyOrderInput) -> Self {
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
pub enum ModifyMultipleOrdersResponse {
    Success(ModifyOrderDetail),
    Failure(BinanceError),
}
