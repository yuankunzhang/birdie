use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/userDataStream",
    Method::DELETE,
    SecurityType::UserStream,
    CloseMarginUserDataStreamEndpoint,
    CloseMarginUserDataStreamParams,
    CloseMarginUserDataStreamResponse
);

/// Close a margin user data stream.
///
/// - Weght: 3000
pub struct CloseMarginUserDataStreamEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CloseMarginUserDataStreamEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseMarginUserDataStreamParams {
    pub listen_key: String,
}

impl CloseMarginUserDataStreamParams {
    pub fn new(listen_key: String) -> Self {
        Self { listen_key }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseMarginUserDataStreamResponse {}
