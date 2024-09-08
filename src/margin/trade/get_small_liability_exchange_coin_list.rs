use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/exchange-small-liability",
    Method::GET,
    SecurityType::UserData,
    GetSmallLiabilityExchangeCoinListEndpoint,
    GetSmallLiabilityExchangeCoinListParams,
    GetSmallLiabilityExchangeCoinListResponse
);

/// Query the coins which can be small liability exchange.
///
/// - Weight: 100
pub struct GetSmallLiabilityExchangeCoinListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetSmallLiabilityExchangeCoinListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmallLiabilityExchangeCoinListParams {
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetSmallLiabilityExchangeCoinListParams {
    pub fn new() -> Self {
        Self {
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type GetSmallLiabilityExchangeCoinListResponse = Vec<SmallLiabilityExchangeCoin>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallLiabilityExchangeCoin {
    pub asset: String,
    pub interest: String,
    pub principal: String,
    pub liability_asset: String,
    pub liability_qty: f64,
}
