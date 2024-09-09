use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/convert/exchangeInfo",
    Method::GET,
    SecurityType::UserData,
    ListAllConvertPairsEndpoint,
    ListAllConvertPairsParams,
    ListAllConvertPairsResponse
);

/// Query for all convertible token pairs and the tokens’ respective upper/lower limits
///
/// - Weight: 20
pub struct ListAllConvertPairsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ListAllConvertPairsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAllConvertPairsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    from_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_asset: Option<String>,
}

impl Default for ListAllConvertPairsParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ListAllConvertPairsParams {
    pub fn new() -> Self {
        Self {
            from_asset: None,
            to_asset: None,
        }
    }

    pub fn from_asset(mut self, from_asset: &str) -> Self {
        self.from_asset = Some(from_asset.to_string());
        self
    }

    pub fn to_asset(mut self, to_asset: &str) -> Self {
        self.to_asset = Some(to_asset.to_string());
        self
    }
}

pub type ListAllConvertPairsResponse = Vec<ConvertPair>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertPair {
    pub from_asset: String,
    pub to_asset: String,
    pub from_asset_min_amount: String,
    pub from_asset_max_amount: String,
    pub to_asset_min_amount: String,
    pub to_asset_max_amount: String,
}
