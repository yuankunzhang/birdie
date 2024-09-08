use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/bnbBurn",
    Method::GET,
    SecurityType::UserData,
    GetBnbBurnStatusEndpoint,
    GetBnbBurnStatusParams,
    GetBnbBurnStatusResponse
);

/// Get BNB Burn Status
///
/// - Weight: 1
pub struct GetBnbBurnStatusEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> GetBnbBurnStatusEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBnbBurnStatusParams {
    recv_window: Option<i64>,
    timestamp: i64,
}

impl GetBnbBurnStatusParams {
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

#[derive(Debug, Deserialize)]
pub struct GetBnbBurnStatusResponse {
    #[serde(rename = "spotBNBBurn")]
    pub spot_bnb_burn: bool,
    #[serde(rename = "interestBNBBurn")]
    pub interest_bnb_burn: String,
}
