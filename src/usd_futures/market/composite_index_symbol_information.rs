use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/indexInfo",
    Method::GET,
    CompositeIndexSymbolInformationEndpoint,
    CompositeIndexSymbolInformationParams,
    CompositeIndexSymbolInformationResponse
);

/// Latest price for a symbol or symbols.
///
/// - Weight: 1
pub struct CompositeIndexSymbolInformationEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CompositeIndexSymbolInformationEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeIndexSymbolInformationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
}

impl Default for CompositeIndexSymbolInformationParams {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositeIndexSymbolInformationParams {
    pub fn new() -> Self {
        Self { symbol: None }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

pub type CompositeIndexSymbolInformationResponse = Vec<CompositeIndexSymbolInformation>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeIndexSymbolInformation {
    pub symbol: String,
    pub time: i64,
    pub component: Vec<BaseAsset>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAsset {
    pub base_asset: String,
    pub quote_asset: String,
    pub weight_in_quantity: String,
    pub weight_in_percentage: String,
}
