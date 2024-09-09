use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        ContingencyType, OrderSide, OrderStatus, OrderType, ResponseType, SecurityType,
        SelfTradePreventionMode, SideEffectType, TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/order/oco",
    Method::POST,
    SecurityType::Trade,
    NewOcoEndpoint,
    NewOcoParams,
    NewOcoResponse
);

/// Send in a new OCO for a margin account.
///
/// - Weight: 6
pub struct NewOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOcoParams {
    symbol: String,
    isolated: Option<String>,
    list_client_order_id: Option<String>,
    side: OrderSide,
    quantity: f64,
    limit_client_order_id: Option<String>,
    price: f64,
    limit_iceberg_qty: Option<f64>,
    stop_client_order_id: Option<String>,
    stop_price: f64,
    stop_limit_price: Option<f64>,
    stop_iceberg_qty: Option<f64>,
    stop_limit_time_in_force: Option<TimeInForce>,
    new_order_resp_type: Option<ResponseType>,
    side_effect_type: Option<SideEffectType>,
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    auto_repay_at_cancel: Option<bool>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOcoParams {
    pub fn new(symbol: &str, side: OrderSide, quantity: f64, price: f64, stop_price: f64) -> Self {
        Self {
            symbol: symbol.to_owned(),
            isolated: None,
            list_client_order_id: None,
            side,
            quantity,
            limit_client_order_id: None,
            price,
            limit_iceberg_qty: None,
            stop_client_order_id: None,
            stop_price,
            stop_limit_price: None,
            stop_iceberg_qty: None,
            stop_limit_time_in_force: None,
            new_order_resp_type: None,
            side_effect_type: None,
            self_trade_prevention_mode: None,
            auto_repay_at_cancel: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn isolated(mut self, isolated: &str) -> Self {
        self.isolated = Some(isolated.to_owned());
        self
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn limit_client_order_id(mut self, limit_client_order_id: &str) -> Self {
        self.limit_client_order_id = Some(limit_client_order_id.to_owned());
        self
    }

    pub fn limit_iceberg_qty(mut self, limit_iceberg_qty: f64) -> Self {
        self.limit_iceberg_qty = Some(limit_iceberg_qty);
        self
    }

    pub fn stop_client_order_id(mut self, stop_client_order_id: &str) -> Self {
        self.stop_client_order_id = Some(stop_client_order_id.to_owned());
        self
    }

    pub fn stop_limit_price(mut self, stop_limit_price: f64) -> Self {
        self.stop_limit_price = Some(stop_limit_price);
        self
    }

    pub fn stop_iceberg_qty(mut self, stop_iceberg_qty: f64) -> Self {
        self.stop_iceberg_qty = Some(stop_iceberg_qty);
        self
    }

    pub fn stop_limit_time_in_force(mut self, stop_limit_time_in_force: TimeInForce) -> Self {
        self.stop_limit_time_in_force = Some(stop_limit_time_in_force);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: ResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn side_effect_type(mut self, side_effect_type: SideEffectType) -> Self {
        self.side_effect_type = Some(side_effect_type);
        self
    }

    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: SelfTradePreventionMode,
    ) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    pub fn auto_repay_at_cancel(mut self, auto_repay_at_cancel: bool) -> Self {
        self.auto_repay_at_cancel = Some(auto_repay_at_cancel);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type NewOcoResponse = MarginOcoOrder;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOcoOrder {
    pub order_list_id: i64,
    pub contingency_type: ContingencyType,
    pub list_status_type: String,
    pub list_order_status: String,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    pub is_isolated: bool,
    #[serde(default)]
    pub orders: Vec<MarginOcoOrderListItem>,
    #[serde(default)]
    pub order_reports: Vec<MarginOcoOrderListReport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOcoOrderListItem {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOcoOrderListReport {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub iceberg_qty: Option<String>,
    pub stop_price: Option<String>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}
