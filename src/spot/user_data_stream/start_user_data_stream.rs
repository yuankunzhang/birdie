use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/api/v3/userDataStream",
    Method::POST,
    SecurityType::UserStream,
    StartUserDataStreamEndpoint,
    StartUserDataStreamParams,
    StartUserDataStreamResponse
);

/// Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent.
///
/// - Weight: 2
/// - Data Source: Memory
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartUserDataStreamResponse {
    pub listen_key: String,
}
