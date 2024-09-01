//! Test connectivity to the Rest API and get the current server time.
//!
//! Weight(IP): 1

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    TimeEndpoint,
    "/api/v3/time",
    Method::GET,
    TimeParams,
    TimeResponse
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeParams {}

impl Default for TimeParams {
    fn default() -> Self {
        TimeParams::new()
    }
}

impl TimeParams {
    pub fn new() -> Self {
        TimeParams {}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeResponse {
    /// Example: 1499827319559
    pub server_time: i64,
}
