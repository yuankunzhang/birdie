use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        futures::{OrderSide, PositionSide},
        SecurityType,
    },
    rest_api::endpoint,
};

endpoint!(
    "/fapi/v1/userTrades",
    Method::GET,
    SecurityType::UserData,
    AccountTradeListEndpoint,
    AccountTradeListParams,
    AccountTradeListResponse
);

/// Get trades for a specific account and symbol.
///
/// - Weight: 5
pub struct AccountTradeListEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> AccountTradeListEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeListParams {
    symbol: String,
    order_id: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    from_id: Option<i64>,
    limit: Option<i64>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl AccountTradeListParams {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn from_id(mut self, from_id: i64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type AccountTradeListResponse = Vec<AccountTradeDetail>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeDetail {
    pub buyer: bool,
    pub commission: String,
    pub commission_asset: String,
    pub id: i64,
    pub maker: bool,
    pub order_id: i64,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub symbol: String,
    pub time: i64,
}
