use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/listen-key",
    Method::PUT,
    SecurityType::UserStream,
    KeepaliveUserDataStreamEndpoint,
    KeepaliveUserDataStreamParams,
    KeepaliveUserDataStreamResponse
);

/// Keepalive a user data stream to prevent a time out.
///
/// - Weight: 1
pub struct KeepaliveUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> KeepaliveUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepaliveUserDataStreamParams {
    pub listen_key: String,
}

impl KeepaliveUserDataStreamParams {
    pub fn new(listen_key: String) -> Self {
        Self { listen_key }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepaliveUserDataStreamResponse {}
