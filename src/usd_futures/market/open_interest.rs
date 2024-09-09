use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/openInterest",
    Method::GET,
    OpenInterestEndpoint,
    OpenInterestParams,
    OpenInterestResponse
);

/// Get present open interest of a specific symbol.
///
/// - Weight: 1
pub struct OpenInterestEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> OpenInterestEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestParams {
    pub symbol: String,
}

impl OpenInterestParams {
    pub fn new(symbol: &str) -> Self {
        OpenInterestParams {
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestResponse {
    pub open_interest: String,
    pub symbol: String,
    pub time: i64,
}
