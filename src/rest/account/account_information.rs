use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    models::Account,
    rest::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/account",
    Method::GET,
    SecurityType::UserData,
    AccountInformationEndpoint,
    AccountInformationParams,
    AccountInformationResponse
);

/// Get current account information.
///
/// - Weight: 20
/// - Data Source: Memory => Database
pub struct AccountInformationEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> AccountInformationEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_zero_balances: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl Default for AccountInformationParams {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountInformationParams {
    pub fn new() -> Self {
        Self {
            omit_zero_balances: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    /// When set to `true`, emits only the non-zero balances of an account.
    /// Default value: `false`.
    pub fn omit_zero_balances(mut self, omit_zero_balances: bool) -> Self {
        self.omit_zero_balances = Some(omit_zero_balances);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type AccountInformationResponse = Account;
