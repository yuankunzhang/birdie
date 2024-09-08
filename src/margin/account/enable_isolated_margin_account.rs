use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/isolated/account",
    Method::POST,
    SecurityType::Trade,
    EnableIsolatedMarginAccountEndpoint,
    EnableIsolatedMarginAccountParams,
    EnableIsolatedMarginAccountResponse
);

/// Enable isolated margin account for a specific symbol(Only supports
/// activation of previously disabled accounts).
///
/// - Weight: 300
pub struct EnableIsolatedMarginAccountEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> EnableIsolatedMarginAccountEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableIsolatedMarginAccountParams {
    symbol: String,
}

impl EnableIsolatedMarginAccountParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableIsolatedMarginAccountResponse {
    pub success: bool,
    pub symbol: String,
}
