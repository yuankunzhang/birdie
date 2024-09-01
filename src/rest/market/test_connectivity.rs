//! Test connectivity to the Rest API.
//!
//! Weight(IP): 1

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
