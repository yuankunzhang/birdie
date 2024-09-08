use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/listen-key",
    Method::DELETE,
    SecurityType::UserStream,
    CloseUserDataStreamEndpoint,
    CloseUserDataStreamParams,
    CloseUserDataStreamResponse
);

/// Close out a user data stream.
///
/// - Weght: 3000
pub struct CloseUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CloseUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseUserDataStreamParams {
    pub listen_key: String,
}

impl CloseUserDataStreamParams {
    pub fn new(listen_key: String) -> Self {
        Self { listen_key }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseUserDataStreamResponse {}
