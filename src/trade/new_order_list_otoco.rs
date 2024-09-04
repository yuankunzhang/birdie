use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{NewOrderRespType, OrderSide, OrderType, SelfTradePreventionMode, TimeInForce},
    rest_api::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/orderList/otoco",
    Method::POST,
    SecurityType::Trade,
    NewOrderListOtocoEndpoint,
    NewOrderListOtocoParams,
    NewOrderListOtocoResponse
);

/// Place an OTOCO.
///
/// - An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list comprised
///   of 3 orders.
/// - The first order is called the working order and must be `LIMIT` or
///   `LIMIT_MAKER`. Initially, only the working order goes on the order book.
///   - The behavior of the working order is the same as the OTO.
/// - OTOCO has 2 pending orders (pending above and pending below), forming an
///   OCO pair. The pending orders are only placed on the order book when the
///   working order gets fully filled.
///   - The rules of the pending above and pending below follow the same rules
///     as the Order List OCO.
/// - OTOCOs count as 3 orders against the order rate limit,
///   `ExchangeMaxNumOrders` filter, and `MaxNumOrders` filter.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderListOtocoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderListOtocoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderListOtocoParams {
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
    pending_side: OrderSide,
    pending_quantity: f64,
    pending_above_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_above_strategy_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending_below_strategy_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderListOtocoParams {
    pub fn new(
        symbol: &str,
        working_type: OrderType,
        working_side: OrderSide,
        working_price: f64,
        working_quantity: f64,
        pending_side: OrderSide,
        pending_quantity: f64,
        pending_above_type: OrderType,
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
            pending_side,
            pending_quantity,
            pending_above_type,
            pending_above_client_order_id: None,
            pending_above_price: None,
            pending_above_stop_price: None,
            pending_above_trailing_delta: None,
            pending_above_iceberg_qty: None,
            pending_above_time_in_force: None,
            pending_above_strategy_id: None,
            pending_above_strategy_type: None,
            pending_below_type: None,
            pending_below_client_order_id: None,
            pending_below_price: None,
            pending_below_stop_price: None,
            pending_below_trailing_delta: None,
            pending_below_iceberg_qty: None,
            pending_below_time_in_force: None,
            pending_below_strategy_id: None,
            pending_below_strategy_type: None,
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

    pub fn pending_above_client_order_id(mut self, pending_above_client_order_id: &str) -> Self {
        self.pending_above_client_order_id = Some(pending_above_client_order_id.to_owned());
        self
    }

    pub fn pending_above_price(mut self, pending_above_price: f64) -> Self {
        self.pending_above_price = Some(pending_above_price);
        self
    }

    pub fn pending_above_stop_price(mut self, pending_above_stop_price: f64) -> Self {
        self.pending_above_stop_price = Some(pending_above_stop_price);
        self
    }

    pub fn pending_above_trailing_delta(mut self, pending_above_trailing_delta: f64) -> Self {
        self.pending_above_trailing_delta = Some(pending_above_trailing_delta);
        self
    }

    pub fn pending_above_iceberg_qty(mut self, pending_above_iceberg_qty: i64) -> Self {
        self.pending_above_iceberg_qty = Some(pending_above_iceberg_qty);
        self
    }

    pub fn pending_above_time_in_force(mut self, pending_above_time_in_force: TimeInForce) -> Self {
        self.pending_above_time_in_force = Some(pending_above_time_in_force);
        self
    }

    pub fn pending_above_strategy_id(mut self, pending_above_strategy_id: i64) -> Self {
        self.pending_above_strategy_id = Some(pending_above_strategy_id);
        self
    }

    pub fn pending_above_strategy_type(mut self, pending_above_strategy_type: i64) -> Self {
        self.pending_above_strategy_type = Some(pending_above_strategy_type);
        self
    }

    pub fn pending_below_type(mut self, pending_below_type: OrderType) -> Self {
        self.pending_below_type = Some(pending_below_type);
        self
    }

    pub fn pending_below_client_order_id(mut self, pending_below_client_order_id: &str) -> Self {
        self.pending_below_client_order_id = Some(pending_below_client_order_id.to_owned());
        self
    }

    pub fn pending_below_price(mut self, pending_below_price: f64) -> Self {
        self.pending_below_price = Some(pending_below_price);
        self
    }

    pub fn pending_below_stop_price(mut self, pending_below_stop_price: f64) -> Self {
        self.pending_below_stop_price = Some(pending_below_stop_price);
        self
    }

    pub fn pending_below_trailing_delta(mut self, pending_below_trailing_delta: f64) -> Self {
        self.pending_below_trailing_delta = Some(pending_below_trailing_delta);
        self
    }

    pub fn pending_below_iceberg_qty(mut self, pending_below_iceberg_qty: i64) -> Self {
        self.pending_below_iceberg_qty = Some(pending_below_iceberg_qty);
        self
    }

    pub fn pending_below_time_in_force(mut self, pending_below_time_in_force: TimeInForce) -> Self {
        self.pending_below_time_in_force = Some(pending_below_time_in_force);
        self
    }

    pub fn pending_below_strategy_id(mut self, pending_below_strategy_id: i64) -> Self {
        self.pending_below_strategy_id = Some(pending_below_strategy_id);
        self
    }

    pub fn pending_below_strategy_type(mut self, pending_below_strategy_type: i64) -> Self {
        self.pending_below_strategy_type = Some(pending_below_strategy_type);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct NewOrderListOtocoResponse;
