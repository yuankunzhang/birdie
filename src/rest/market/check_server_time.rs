//! Test connectivity to the Rest API and get the current server time.
//!
//! Weight(IP): 1

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    "/api/v3/time",
    Method::GET,
    CheckServerTimeEndpoint,
    CheckServerTimeParams,
    CheckServerTimeResponse
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckServerTimeParams {}

impl Default for CheckServerTimeParams {
    fn default() -> Self {
        CheckServerTimeParams::new()
    }
}

impl CheckServerTimeParams {
    pub fn new() -> Self {
        CheckServerTimeParams {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckServerTimeResponse {
    /// Example: 1499827319559
    pub server_time: i64,
}
