#![allow(dead_code)]

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{ed25519_signature, Params, Response};

use super::WebSocketApiError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LogonParams {
    api_key: String,
    signature: String,
    timestamp: i64,
}

impl Params for LogonParams {}

impl LogonParams {
    pub(super) fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            signature: String::new(),
            timestamp: 0,
        }
    }

    pub(super) fn sign(&mut self, key: &str) -> Result<(), WebSocketApiError> {
        let timestamp = Timestamp::now().as_millisecond();
        let data = format!("apiKey={}&timestamp={timestamp}", self.api_key);
        let signature = ed25519_signature(key, &data)?;

        self.timestamp = timestamp;
        self.signature = signature;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LogonResponse {
    api_key: String,
    authorized_since: i64,
    connected_since: i64,
    return_rate_limits: bool,
    server_time: i64,
}

impl Response for LogonResponse {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LogoutParams {}

impl Params for LogoutParams {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LogoutResponse {}

impl Response for LogoutResponse {}
