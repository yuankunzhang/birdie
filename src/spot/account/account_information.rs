use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint, web_socket_api::web_socket};

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
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AccountInformationEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
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

/// Account information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Example: `15`
    pub maker_commission: i64,
    /// Example: `15`
    pub taker_commission: i64,
    /// Example: `0`
    pub buyer_commission: i64,
    /// Example: `0`
    pub seller_commission: i64,
    pub commission_rates: CommissionRate,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    /// Example: `False`
    pub brokered: bool,
    /// Example: `False`
    pub require_self_trade_prevention: bool,
    /// Example: `False`
    pub prevent_sor: bool,
    /// Example: `123456789`
    pub update_time: i64,
    /// Example: `"SPOT"`
    pub account_type: String,
    pub balances: Vec<Balance>,
    pub permissions: Vec<String>,
    /// Example: `354937868`
    pub uid: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRate {
    /// Example: "0.00150000"
    pub maker: String,
    /// Example: "0.00150000"
    pub taker: String,
    /// Example: "0.00000000"
    pub buyer: String,
    /// Example: "0.00000000"
    pub seller: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Example: `"BTC"`
    pub asset: String,
    /// Example: "4723846.89208129"
    pub free: String,
    /// Example: "0.00000000"
    pub locked: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    pub enabled_for_account: bool,
    pub enabled_for_symbol: bool,
    pub discount_asset: Option<String>,
    pub discount: String,
}

web_socket!(
    "account.status",
    AccountInformationWebSocket,
    AccountInformationParams,
    AccountInformationResponse
);

pub struct AccountInformationWebSocket<'w> {
    client: &'w crate::web_socket_api::WebSocketApiClient,
}

impl<'w> AccountInformationWebSocket<'w> {
    pub fn new(client: &'w crate::web_socket_api::WebSocketApiClient) -> Self {
        Self { client }
    }
}
