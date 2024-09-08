use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/allAssets",
    Method::GET,
    SecurityType::MarketData,
    GetAllMarginAssetsEndpoint,
    GetAllMarginAssetsParams,
    GetAllMarginAssetsResponse
);

/// Get All Margin Assets.
///
/// - Weight: 1
pub struct GetAllMarginAssetsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetAllMarginAssetsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllMarginAssetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<String>,
}

impl GetAllMarginAssetsParams {
    pub fn new() -> Self {
        Self { asset: None }
    }

    pub fn asset(mut self, asset: String) -> Self {
        self.asset = Some(asset);
        self
    }
}

pub type GetAllMarginAssetsResponse = Vec<MarginAsset>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAsset {
    pub asset_full_name: String,
    pub asset_name: String,
    pub is_borrowable: bool,
    pub is_mortgageable: bool,
    pub user_min_borrow: String,
    pub user_min_repay: String,
    pub delist_time: Option<i64>,
}
