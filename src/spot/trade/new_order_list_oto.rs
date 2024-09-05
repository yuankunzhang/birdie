use jiff::Timestamp;
use reqwest::Method;
use serde::Serialize;

use crate::{
    enums::{
        NewOrderRespType, OrderSide, OrderType, SecurityType, SelfTradePreventionMode, TimeInForce,
    },
    rest_api::{Endpoint, Params, RestApiClient},
};

use super::OrderListResult;

impl Endpoint for NewOrderListOtoEndpoint<'_> {
    type Response = NewOrderListOtoResponse;
    type Params = NewOrderListOtoParams;

    fn client(&self) -> &RestApiClient {
        self.client
    }

    fn path(&self) -> &str {
        "/api/v3/orderList/oto"
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn security_type(&self) -> SecurityType {
        SecurityType::Trade
    }
}

impl Params for NewOrderListOtoParams {}

/// Place an OTO.
///
/// - An OTO (One-Triggers-the-Other) is an order list comprised of 2 orders.
/// - The first order is called the working order and must be `LIMIT` or
///   `LIMIT_MAKER`. Initially, only the working order goes on the order book.
/// - The /// second order is called the pending order. It can be any order
///   type except for `MARKET` orders using parameter `quote_order_qty`. The
///   pending order is only placed on the order book when the working order
///   gets fully filled.
/// - If either the working order or the pending order is cancelled
///   individually, the other order in the order list will also be canceled or
///   expired.
/// - When the order list is placed, if the working order gets immediately fully
///   filled, the placement response will show the working order as `FILLED` but
///   the pending order will still appear as `PENDING_NEW`. You need to query
///   the status of the pending order again to see its updated status.
/// - OTOs count as 2 orders against the order rate limit,
///   `ExchangeMaxNumOrders` filter and `MaxNumOrders` filter.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderListOtoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderListOtoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderListOtoParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<NewOrderRespType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    working_type: OrderType,
    working_side: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_client_order_id: Option<String>,
    working_price: f64,
    working_quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    working_strategy_type: Option<i64>,
    pending_type: OrderType,
    pending_side: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_client_order_id: Option<String>,
    pending_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_trailing_delta: Option<f64>,
    pending_quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_strategy_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderListOtoParams {
    pub fn new(
        symbol: &str,
        working_type: OrderType,
        working_side: OrderSide,
        working_price: f64,
        working_quantity: f64,
        pending_type: OrderType,
        pending_side: OrderSide,
        pending_price: f64,
        pending_quantity: f64,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            list_client_order_id: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            working_type,
            working_side,
            working_client_order_id: None,
            working_price,
            working_quantity,
            working_iceberg_qty: None,
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            pending_type,
            pending_side,
            pending_client_order_id: None,
            pending_price,
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_quantity,
            pending_iceberg_qty: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderRespType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: SelfTradePreventionMode,
    ) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    pub fn working_client_order_id(mut self, working_client_order_id: &str) -> Self {
        self.working_client_order_id = Some(working_client_order_id.to_owned());
        self
    }

    pub fn working_iceberg_qty(mut self, working_iceberg_qty: i64) -> Self {
        self.working_iceberg_qty = Some(working_iceberg_qty);
        self
    }

    pub fn working_time_in_force(mut self, working_time_in_force: TimeInForce) -> Self {
        self.working_time_in_force = Some(working_time_in_force);
        self
    }

    pub fn working_strategy_id(mut self, working_strategy_id: i64) -> Self {
        self.working_strategy_id = Some(working_strategy_id);
        self
    }

    pub fn working_strategy_type(mut self, working_strategy_type: i64) -> Self {
        self.working_strategy_type = Some(working_strategy_type);
        self
    }

    pub fn pending_client_order_id(mut self, pending_client_order_id: &str) -> Self {
        self.pending_client_order_id = Some(pending_client_order_id.to_owned());
        self
    }

    pub fn pending_stop_price(mut self, pending_stop_price: f64) -> Self {
        self.pending_stop_price = Some(pending_stop_price);
        self
    }

    pub fn pending_trailing_delta(mut self, pending_trailing_delta: f64) -> Self {
        self.pending_trailing_delta = Some(pending_trailing_delta);
        self
    }

    pub fn pending_iceberg_qty(mut self, pending_iceberg_qty: i64) -> Self {
        self.pending_iceberg_qty = Some(pending_iceberg_qty);
        self
    }

    pub fn pending_time_in_force(mut self, pending_time_in_force: TimeInForce) -> Self {
        self.pending_time_in_force = Some(pending_time_in_force);
        self
    }

    pub fn pending_strategy_id(mut self, pending_strategy_id: i64) -> Self {
        self.pending_strategy_id = Some(pending_strategy_id);
        self
    }

    pub fn pending_strategy_type(mut self, pending_strategy_type: i64) -> Self {
        self.pending_strategy_type = Some(pending_strategy_type);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type NewOrderListOtoResponse = OrderListResult;
