use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{rest_api::endpoint, web_socket_api::web_socket};

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
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> TestConnectivityEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
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

web_socket!(
    "ping",
    TestConnectivityWebSocket,
    TestConnectivityParams,
    TestConnectivityResponse
);

pub struct TestConnectivityWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> TestConnectivityWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
