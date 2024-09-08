use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{enums::SecurityType, rest_api::endpoint};

endpoint!(
    "/sapi/v1/margin/borrow-repay",
    Method::POST,
    SecurityType::Margin,
    MarginAccountBorrowRepayEndpoint,
    MarginAccountBorrowRepayParams,
    MarginAccountBorrowRepayResponse
);

/// Margin account borrow/repay(MARGIN).
///
/// - Weight: 1500
pub struct MarginAccountBorrowRepayEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> MarginAccountBorrowRepayEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccountBorrowRepayParams {
    asset: String,
    is_isolated: String,
    symbol: String,
    amount: String,
    r#type: String,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl MarginAccountBorrowRepayParams {
    pub fn new(asset: &str, is_isolated: &str, symbol: &str, amount: &str, r#type: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            is_isolated: is_isolated.to_owned(),
            symbol: symbol.to_owned(),
            amount: amount.to_owned(),
            r#type: r#type.to_owned(),
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccountBorrowRepayResponse {
    pub tran_id: i64,
}
