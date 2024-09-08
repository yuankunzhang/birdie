use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/userDataStream/isolated",
    Method::POST,
    SecurityType::UserStream,
    StartIsolatedMarginUserDataStreamEndpoint,
    StartIsolatedMarginUserDataStreamParams,
    StartIsolatedMarginUserDataStreamResponse
);

/// Start a new margin user data stream. The stream will close after 60 minutes
/// unless a keepalive is sent. If the account has an active listenKey, that
/// listenKey will be returned and its validity will be extended for 60 minutes.
///
/// - Weight: 1
pub struct StartIsolatedMarginUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> StartIsolatedMarginUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartIsolatedMarginUserDataStreamParams {}

impl Default for StartIsolatedMarginUserDataStreamParams {
    fn default() -> Self {
        Self::new()
    }
}

impl StartIsolatedMarginUserDataStreamParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartIsolatedMarginUserDataStreamResponse {
    pub listen_key: String,
}
