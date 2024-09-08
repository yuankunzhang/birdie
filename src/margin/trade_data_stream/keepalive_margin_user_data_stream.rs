use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/userDataStream",
    Method::PUT,
    SecurityType::UserStream,
    KeepaliveMarginUserDataStreamEndpoint,
    KeepaliveMarginUserDataStreamParams,
    KeepaliveMarginUserDataStreamResponse
);

/// Keepalive a margin user data stream to prevent a time out.
///
/// The stream will close after 60 minutes unless a keepalive is sent. If the
/// account has an active listenKey, that listenKey will be returned and its
/// validity will be extended for 60 minutes.
///
/// - Weight: 1
pub struct KeepaliveMarginUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> KeepaliveMarginUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepaliveMarginUserDataStreamParams {
    pub listen_key: String,
}

impl KeepaliveMarginUserDataStreamParams {
    pub fn new(listen_key: String) -> Self {
        Self { listen_key }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepaliveMarginUserDataStreamResponse {}
