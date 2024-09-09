use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest_api::endpoint;

endpoint!(
    "/fapi/v1/constituent",
    Method::GET,
    QueryIndexPriceConstituentsEndpoint,
    QueryIndexPriceConstituentsParams,
    QueryIndexPriceConstituentsResponse
);

/// Query index price constituents
///
/// - Weight: 2
pub struct QueryIndexPriceConstituentsEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryIndexPriceConstituentsEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIndexPriceConstituentsParams {
    pub symbol: String,
}

impl QueryIndexPriceConstituentsParams {
    pub fn new(symbol: &str) -> Self {
        QueryIndexPriceConstituentsParams {
            symbol: symbol.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIndexPriceConstituentsResponse {
    pub symbol: String,
    pub time: i64,
    pub constituents: Vec<Constituent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constituent {
    pub exchange: String,
    pub symbol: String,
}
