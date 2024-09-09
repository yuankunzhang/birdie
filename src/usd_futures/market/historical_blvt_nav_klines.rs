use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::{KlineInterval, SecurityType},
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::Kline;

impl Params for HistoricalBlvtNavKlinesParams {}

impl Endpoint for HistoricalBlvtNavKlinesEndpoint<'_> {
    type Params = HistoricalBlvtNavKlinesParams;
    type Response = HistoricalBlvtNavKlinesResponse;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/fapi/v1/lvtKlines"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::None
    }
}

/// The BLVT NAV system is based on Binance Futures, so the endpoint is based on fapi
///
/// - Weight: 1
pub struct HistoricalBlvtNavKlinesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> HistoricalBlvtNavKlinesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBlvtNavKlinesParams {
    symbol: String,
    interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl HistoricalBlvtNavKlinesParams {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub type HistoricalBlvtNavKlinesResponse = Vec<Kline>;
