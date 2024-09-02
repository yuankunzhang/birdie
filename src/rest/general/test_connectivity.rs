use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    "/api/v3/ping",
    Method::GET,
    TestConnectivityEndpoint,
    TestConnectivityParams,
    TestConnectivityResponse
);

/// Test connectivity to the Rest API.
///
/// - Weight: 1
/// - Data Source: Memory
pub struct TestConnectivityEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> TestConnectivityEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestConnectivityParams {}

impl Default for TestConnectivityParams {
    fn default() -> Self {
        Self::new()
    }
}

impl TestConnectivityParams {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestConnectivityResponse {}
