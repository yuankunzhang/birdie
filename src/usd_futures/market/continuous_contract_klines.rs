use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::{futures::ContractType, KlineInterval, SecurityType},
    rest_api::{Endpoint, RestApiClient},
    Params,
};

use super::Kline;

impl Params for ContinuousContractKlinesParams {}

impl Endpoint for ContinuousContractKlinesEndpoint<'_> {
    type Params = ContinuousContractKlinesParams;
    type Response = ContinuousContractKlinesResponse;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/fapi/v1/continuousKlines"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::None
    }
}

/// Kline/candlestick bars for a symbol. ContinuousContractKlines are uniquely identified by their
/// open time.
///
/// - Weight:
///   - limit [1,100): 1
///   - limit [100,500): 2
///   - limit [500,1000): 5
///   - limit [1000,): 10
pub struct ContinuousContractKlinesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ContinuousContractKlinesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousContractKlinesParams {
    pair: String,
    contract_type: ContractType,
    interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

impl ContinuousContractKlinesParams {
    pub fn new(pair: &str, contract_type: ContractType, interval: KlineInterval) -> Self {
        Self {
            pair: pair.to_owned(),
            contract_type,
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

pub type ContinuousContractKlinesResponse = Vec<Kline>;
