use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/convert/orderStatus",
    Method::GET,
    SecurityType::UserData,
    OrderStatusEndpoint,
    OrderStatusParams,
    OrderStatusResponse
);

/// Query order status by order ID.
///
/// - Weight: 50
pub struct OrderStatusEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> OrderStatusEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusParams {
    order_id: Option<String>,
    quote_id: Option<String>,
}

impl OrderStatusParams {
    pub fn new() -> Self {
        Self {
            order_id: None,
            quote_id: None,
        }
    }

    pub fn order_id(mut self, order_id: impl Into<String>) -> Self {
        self.quote_id = Some(order_id.into());
        self
    }

    pub fn quote_id(mut self, quote_id: impl Into<String>) -> Self {
        self.quote_id = Some(quote_id.into());
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusResponse {
    pub order_id: String,
    pub order_status: String,
    pub from_asset: String,
    pub from_amount: String,
    pub to_asset: String,
    pub to_amount: String,
    pub ratio: String,
    pub inverse_ratio: String,
    pub create_time: i64,
}
