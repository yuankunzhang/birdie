use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

use super::{CommissionRate, Discount};

endpoint!(
    "/api/v3/account/commission",
    Method::GET,
    SecurityType::UserData,
    QueryCommissionRatesEndpoint,
    QueryCommissionRatesParams,
    QueryCommissionRatesResponse
);

/// Get current account commission rates.
///
/// - Weight: 20
/// - Data Source: Database
pub struct QueryCommissionRatesEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> QueryCommissionRatesEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCommissionRatesParams {
    symbol: String,
    timestamp: i64,
}

impl QueryCommissionRatesParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            timestamp: Timestamp::now().as_millisecond(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCommissionRatesResponse {
    pub symbol: String,
    pub standard_commission: CommissionRate,
    pub tax_commission: CommissionRate,
    pub discount: Discount,
}
