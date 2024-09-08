use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/capital-flow",
    Method::GET,
    SecurityType::UserData,
    QueryCrossIsolatedMarginCapitalFlowEndpoint,
    QueryCrossIsolatedMarginCapitalFlowParams,
    QueryCrossIsolatedMarginCapitalFlowResponse
);

/// Query Cross Isolated Margin Capital Flow.
///
/// - Weight: 100
pub struct QueryCrossIsolatedMarginCapitalFlowEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCrossIsolatedMarginCapitalFlowEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCrossIsolatedMarginCapitalFlowParams {
    asset: Option<String>,
    symbol: Option<String>,
    r#type: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    from_id: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl QueryCrossIsolatedMarginCapitalFlowParams {
    pub fn new() -> Self {
        Self {
            asset: None,
            symbol: None,
            r#type: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn asset(mut self, asset: String) -> Self {
        self.asset = Some(asset);
        self
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn r#type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type QueryCrossIsolatedMarginCapitalFlowResponse = Vec<CrossIsolatedMarginCapitalFlow>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossIsolatedMarginCapitalFlow {
    pub id: i64,
    pub tran_id: i64,
    pub timestamp: i64,
    pub asset: String,
    pub symbol: Option<String>,
    pub r#type: String,
    pub amount: String,
}
