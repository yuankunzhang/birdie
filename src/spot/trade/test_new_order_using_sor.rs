use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint, web_socket_api::web_socket};

use super::CommissionRates;

endpoint!(
    "/api/v3/order/test",
    Method::POST,
    SecurityType::Trade,
    TestNewOrderUsingSorEndpoint,
    TestNewOrderUsingSorParams,
    TestNewOrderUsingSorResponse
);

/// Test new order creation and signature/recvWindow using smart order routing
/// (SOR). Creates and validates a new order but does not send it into the
/// matching engine.
///
/// - Weight:
///     - Without `computeCommissionRates`: 1
///     - With `computeCommissionRates`: 20
/// - Data Source: Memory
pub struct TestNewOrderUsingSorEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TestNewOrderUsingSorEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderUsingSorParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_commission_rates: Option<bool>,
}

impl TestNewOrderUsingSorParams {
    pub fn new() -> Self {
        Self {
            compute_commission_rates: None,
        }
    }

    /// Default: false.
    pub fn compute_commission_rates(mut self, compute_commission_rates: bool) -> Self {
        self.compute_commission_rates = Some(compute_commission_rates);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TestNewOrderUsingSorResponse {
    Empty,
    CommissionRates(CommissionRates),
}

web_socket!(
    "sor.order.test",
    TestNewOrderUsingSorWebSocket,
    TestNewOrderUsingSorParams,
    TestNewOrderUsingSorResponse
);

pub struct TestNewOrderUsingSorWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> TestNewOrderUsingSorWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
