use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/leverageBracket",
    Method::GET,
    SecurityType::MarketData,
    QueryLiabilityCoinLeverageBracketEndpoint,
    QueryLiabilityCoinLeverageBracketParams,
    QueryLiabilityCoinLeverageBracketResponse
);

/// Liability Coin Leverage Bracket in Cross Margin Pro Mode.
///
/// - Weight: 1
pub struct QueryLiabilityCoinLeverageBracketEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryLiabilityCoinLeverageBracketEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLiabilityCoinLeverageBracketParams {}

impl QueryLiabilityCoinLeverageBracketParams {
    pub fn new() -> Self {
        Self {}
    }
}

pub type QueryLiabilityCoinLeverageBracketResponse = Vec<LiabilityCoinLeverageBracket>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiabilityCoinLeverageBracket {
    pub asset_names: Vec<String>,
    pub rank: i64,
    pub brackets: Vec<Bracket>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    pub leverage: i64,
    pub max_debt: String,
    pub maintenance_margin_rate: String,
    pub initial_margin_rate: String,
    pub fast_num: String,
}
