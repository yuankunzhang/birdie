use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::enums::SecurityType;
use crate::rest_api::endpoint;
use crate::spot::account::Discount;

use super::NewOrderParams;

endpoint!(
    "/api/v3/order/test",
    Method::POST,
    SecurityType::Trade,
    TestNewOrderEndpoint,
    TestNewOrderParams,
    TestNewOrderResponse
);

/// Test new order creation and signature/recvWindow long. Creates and validates
/// a new order but does not send it into the matching engine.
///
/// - Weight:
///     - Without `computeCommissionRates`: 1
///     - With `computeCommissionRates`: 20
/// - Data Source: Memory
pub struct TestNewOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TestNewOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderParams {
    #[serde(flatten)]
    new_order_params: NewOrderParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_commission_rates: Option<bool>,
}

impl TestNewOrderParams {
    pub fn new(new_order_params: NewOrderParams) -> Self {
        Self {
            new_order_params,
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
pub enum TestNewOrderResponse {
    Empty,
    CommissionRates(CommissionRates),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    pub standard_commission_for_order: CommissionForOrder,
    pub tax_commission_for_order: CommissionForOrder,
    pub discount: Discount,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionForOrder {
    pub maker: i64,
    pub taker: i64,
}
