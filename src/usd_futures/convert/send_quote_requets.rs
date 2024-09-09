use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/convert/getQuote",
    Method::POST,
    SecurityType::UserData,
    SendQuoteRequestEndpoint,
    SendQuoteRequestParams,
    SendQuoteRequestResponse
);

/// Request a quote for the requested token pairs.
///
/// - Weight: 50
pub struct SendQuoteRequestEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> SendQuoteRequestEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendQuoteRequestParams {
    from_asset: String,
    to_asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl SendQuoteRequestParams {
    pub fn new(from_asset: &str, to_asset: &str) -> Self {
        Self {
            from_asset: from_asset.to_owned(),
            to_asset: to_asset.to_owned(),
            from_amount: None,
            to_amount: None,
            valid_time: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn from_amount(mut self, from_amount: f64) -> Self {
        self.from_amount = Some(from_amount);
        self
    }

    pub fn to_amount(mut self, to_amount: f64) -> Self {
        self.to_amount = Some(to_amount);
        self
    }

    pub fn valid_time(mut self, valid_time: &str) -> Self {
        self.valid_time = Some(valid_time.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendQuoteRequestResponse {
    pub quote_id: String,
    pub ratio: String,
    pub inverse_ratio: String,
    pub valid_timestamp: i64,
    pub to_amount: String,
    pub from_amount: String,
}
