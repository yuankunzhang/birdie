use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/fapi/v1/convert/acceptQuote",
    Method::POST,
    SecurityType::UserData,
    AcceptQuoteEndpoint,
    AcceptQuoteParams,
    AcceptQuoteResponse
);

/// Accept the offered quote by quote ID.
///
/// - Weight: 200
pub struct AcceptQuoteEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AcceptQuoteEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptQuoteParams {
    quote_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl AcceptQuoteParams {
    pub fn new(quote_id: &str) -> Self {
        Self {
            quote_id: quote_id.to_owned(),
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
#[serde(rename_all = "camelCase")]
pub struct AcceptQuoteResponse {
    pub order_id: String,
    pub create_time: i64,
    pub order_status: String,
}
