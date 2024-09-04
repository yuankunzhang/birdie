use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::{endpoint, SecurityType};

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
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderUsingSorResponse;
