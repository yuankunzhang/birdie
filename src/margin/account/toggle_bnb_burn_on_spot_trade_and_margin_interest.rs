use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/bnbBurn",
    Method::POST,
    SecurityType::UserData,
    ToggleBnbBurnOnSpotTradeAndMarginInterestEndpoint,
    ToggleBnbBurnOnSpotTradeAndMarginInterestParams,
    ToggleBnbBurnOnSpotTradeAndMarginInterestResponse
);

/// Toggle BNB Burn On Spot Trade And Margin Interest.
///
/// - Weight: 1
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> ToggleBnbBurnOnSpotTradeAndMarginInterestEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestParams {
    #[serde(rename = "spotBNBBurn")]
    spot_bnb_burn: Option<String>,
    #[serde(rename = "interestBNBBurn")]
    interest_bnb_burn: Option<String>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl ToggleBnbBurnOnSpotTradeAndMarginInterestParams {
    pub fn new() -> Self {
        Self {
            spot_bnb_burn: None,
            interest_bnb_burn: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn spot_bnb_burn(mut self, spot_bnb_burn: String) -> Self {
        self.spot_bnb_burn = Some(spot_bnb_burn);
        self
    }

    pub fn interest_bnb_burn(mut self, interest_bnb_burn: String) -> Self {
        self.interest_bnb_burn = Some(interest_bnb_burn);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
    #[serde(rename = "spotBNBBurn")]
    pub spot_bnb_burn: bool,
    #[serde(rename = "interestBNBBurn")]
    pub interest_bnb_burn: bool,
}
