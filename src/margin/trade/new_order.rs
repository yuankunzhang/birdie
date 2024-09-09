use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        OrderSide, OrderStatus, OrderType, ResponseType, SecurityType, SelfTradePreventionMode,
        SideEffectType, TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/sapi/v1/margin/order",
    Method::POST,
    SecurityType::Trade,
    NewOrderEndpoint,
    NewOrderParams,
    NewOrderResponse
);

/// Post a new order for margin account.
///
/// - Weight: 6
pub struct NewOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams {
    symbol: String,
    is_isolated: Option<bool>,
    side: OrderSide,
    r#type: OrderType,
    quantity: Option<f64>,
    quote_order_qty: Option<f64>,
    price: Option<f64>,
    stop_price: Option<f64>,
    new_client_order_id: Option<String>,
    iceberg_qty: Option<f64>,
    new_order_resp_type: Option<ResponseType>,
    side_effect_type: Option<SideEffectType>,
    time_in_force: Option<TimeInForce>,
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    auto_repay_at_cancel: Option<bool>,
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderParams {
    pub fn new(symbol: &str, side: OrderSide, r#type: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            is_isolated: None,
            side,
            r#type,
            quantity: None,
            quote_order_qty: None,
            price: None,
            stop_price: None,
            new_client_order_id: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            side_effect_type: None,
            time_in_force: None,
            self_trade_prevention_mode: None,
            auto_repay_at_cancel: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
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

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewOrderResponse {
    Ack(Box<NewMarginOrderAck>),
    Result(Box<NewMarginOrderResult>),
    Full(Box<NewMarginOrderFull>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMarginOrderAck {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub is_isolated: bool,
    pub transact_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMarginOrderResult {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: OrderSide,
    pub is_isolated: bool,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewMarginOrderFull {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    pub r#type: OrderType,
    pub side: String,
    pub margin_buy_borrow_amount: Option<i64>,
    pub margin_buy_borrow_asset: Option<String>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub fills: Vec<MarginOrderFill>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderFill {
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub trade_id: i64,
}
