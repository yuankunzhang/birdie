use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/api/v3/userDataStream",
    Method::PUT,
    SecurityType::UserStream,
    KeepaliveUserDataStreamEndpoint,
    KeepaliveUserDataStreamParams,
    KeepaliveUserDataStreamResponse
);

/// Keepalive a user data stream to prevent a time out. User data streams will
/// close after 60 minutes. It's recommended to send a ping about every 30
/// minutes.
///
/// - Weight: 2
/// - Data Source: Memory
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
