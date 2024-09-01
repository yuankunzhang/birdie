use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::{Endpoint, Params, Response, RestClient, RestError};

/// Test connectivity to the Rest API and get the current server time.
///
/// Weight(IP): 1
pub struct TimeEndpoint<'r> {
    client: &'r RestClient,
}

impl<'r> TimeEndpoint<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Endpoint for TimeEndpoint<'_> {
    type Params = TimeParams;
    type Response = TimeResponse;

    fn path(&self) -> &str {
        "api/v3/time"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    async fn request(&self, params: Self::Params) -> Result<Self::Response, RestError> {
        self.client.request(Method::GET, self.path(), params).await
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeParams {}

impl Params for TimeParams {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeResponse {
    /// Example: 1499827319559
    pub server_time: i64,
}

impl Response for TimeResponse {}
