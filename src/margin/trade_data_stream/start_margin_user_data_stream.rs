use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/userDataStream",
    Method::POST,
    SecurityType::UserStream,
    StartMarginUserDataStreamEndpoint,
    StartMarginUserDataStreamParams,
    StartMarginUserDataStreamResponse
);

/// Start a new margin user data stream. The stream will close after 60 minutes
/// unless a keepalive is sent. If the account has an active listenKey, that
/// listenKey will be returned and its validity will be extended for 60 minutes.
///
/// - Weight: 1
pub struct StartMarginUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> StartMarginUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartMarginUserDataStreamParams {}

impl Default for StartMarginUserDataStreamParams {
    fn default() -> Self {
        Self::new()
    }
}

impl StartMarginUserDataStreamParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartMarginUserDataStreamResponse {
    pub listen_key: String,
}
