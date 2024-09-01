//! Test connectivity to the Rest API.
//!
//! Weight(IP): 1

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::rest::endpoint;

endpoint!(
    PingEndpoint,
    "/api/v3/ping",
    Method::GET,
    PingParams,
    PingResponse
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingParams {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {}
