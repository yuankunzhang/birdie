use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/crossMarginCollateralRatio",
    Method::GET,
    SecurityType::MarketData,
    CrossMarginCollateralRatioEndpoint,
    CrossMarginCollateralRatioParams,
    CrossMarginCollateralRatioResponse
);

/// Cross margin collateral ratio.
///
/// - Weight: 100
pub struct CrossMarginCollateralRatioEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CrossMarginCollateralRatioEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginCollateralRatioParams {}

impl CrossMarginCollateralRatioParams {
    pub fn new() -> Self {
        Self {}
    }
}

pub type CrossMarginCollateralRatioResponse = Vec<CrossMarginCollateralRatio>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginCollateralRatio {
    pub collaterals: Vec<Collateral>,
    pub asset_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    #[serde(default)]
    pub min_used_value: String,
    #[serde(default)]
    pub max_used_value: String,
    #[serde(default)]
    pub discount_rate: String,
}
