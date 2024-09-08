use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/max-leverage",
    Method::GET,
    SecurityType::UserData,
    AdjustCrossMarginMaxLeverageEndpoint,
    AdjustCrossMarginMaxLeverageParams,
    AdjustCrossMarginMaxLeverageResponse
);

/// Adjust cross margin max leverage.
///
/// - Weight: 3000
pub struct AdjustCrossMarginMaxLeverageEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AdjustCrossMarginMaxLeverageEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCrossMarginMaxLeverageParams {
    max_leverage: i64,
}

impl AdjustCrossMarginMaxLeverageParams {
    pub fn new(max_leverage: i64) -> Self {
        Self { max_leverage }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustCrossMarginMaxLeverageResponse {
    pub success: bool,
}
