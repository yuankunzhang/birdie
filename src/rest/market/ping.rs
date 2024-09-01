use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::{Endpoint, Params, Response, RestClient, RestError};

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

    fn path(&self) -> &str {
        "api/v3/ping"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    async fn request(&self, params: Self::Params) -> Result<Self::Response,RestError> {
        self.client.request(Method::GET, self.path(), params).await
    }
}

#[derive(Debug, Serialize)]
pub struct PingParams {}

impl Params for PingParams {}

#[derive(Debug, Deserialize)]
pub struct PingResponse {}

impl Response for PingResponse {}
