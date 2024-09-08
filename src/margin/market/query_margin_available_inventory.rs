use std::collections::HashMap;

use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/available-inventory",
    Method::GET,
    SecurityType::MarketData,
    QueryMarginAvailableInventoryEndpoint,
    QueryMarginAvailableInventoryParams,
    QueryMarginAvailableInventoryResponse
);

/// Margin available Inventory query.
///
/// - Weight: 50
pub struct QueryMarginAvailableInventoryEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryMarginAvailableInventoryEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMarginAvailableInventoryParams {
    r#type: String,
    timestamp: i64,
}

impl QueryMarginAvailableInventoryParams {
    pub fn new(r#type: &str) -> Self {
        Self {
            r#type: r#type.to_owned(),
            timestamp: Timestamp::now().as_millisecond(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMarginAvailableInventoryResponse {
    pub assets: HashMap<String, String>,
    pub update_time: i64,
}
