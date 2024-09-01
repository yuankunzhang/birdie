use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::{Endpoint, Params, Response, RestClient};

/// Test connectivity to the Rest API.
///
/// Weight(IP): 1
pub struct PingEndpoint<'r> {
    client: &'r RestClient,
}

impl<'r> PingEndpoint<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Endpoint for PingEndpoint<'_> {
    type Params = PingParams;
    type Response = PingResponse;

    fn client(&self) -> &RestClient {
        self.client
    }

    fn path(&self) -> &str {
        "api/v3/ping"
    }

    fn method(&self) -> Method {
        Method::GET
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingParams {}

impl Params for PingParams {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {}

impl Response for PingResponse {}
