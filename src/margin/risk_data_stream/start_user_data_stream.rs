use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/listen-key",
    Method::POST,
    SecurityType::UserStream,
    StartUserDataStreamEndpoint,
    StartUserDataStreamParams,
    StartUserDataStreamResponse
);

/// Start a new user data stream.
///
/// - Weight: 1
pub struct StartUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> StartUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartUserDataStreamParams {}

impl Default for StartUserDataStreamParams {
    fn default() -> Self {
        Self::new()
    }
}

impl StartUserDataStreamParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartUserDataStreamResponse {
    pub listen_key: String,
}
