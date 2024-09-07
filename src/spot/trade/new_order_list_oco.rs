use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        ContingencyType, NewOrderRespType, OrderListOrderStatus, OrderListStatus, OrderSide,
        OrderStatus, OrderType, SecurityType, SelfTradePreventionMode, TimeInForce,
    },
    rest_api::endpoint,
};

endpoint!(
    "/api/v3/orderList/oco",
    Method::POST,
    SecurityType::Trade,
    NewOrderListOcoEndpoint,
    NewOrderListOcoParams,
    NewOrderListOcoResponse
);

/// Send in an one-cancels-the-other (OCO) pair, where activation of one order
/// immediately cancels the other.
///
/// - An OCO has 2 legs called the above leg and below leg.
/// - One of the legs must be a [`OrderType::LimitMaker`] order and the other
///   leg must be [`OrderType::StopLoss`] or [`OrderType::StopLossLimit`] order.
/// - Price restrictions:
///     - If the OCO is on the [`OrderSide::Sell`] side:
///       [`OrderType::LimitMaker`] price > Last Traded Price > `stop_price`
///     - If the OCO is on the [`OrderSide::Buy`] side:
///       [`OrderType::LimitMaker`] price < Last Traded Price < `stop_price`
/// - OCOs add 2 orders to the unfilled order count, `ExchangeMaxNumOrders`
///   filter and `MaxNumOrders` filter.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderListOcoEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderListOcoEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderListOcoParams {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<String>,
    side: OrderSide,
    quantity: Option<f64>,
    above_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    above_strategy_type: Option<i64>,
    below_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_iceberg_qty: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    below_strategy_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<NewOrderRespType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderListOcoParams {
    pub fn new(
        symbol: &str,
        side: OrderSide,
        above_type: OrderType,
        below_type: OrderType,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            list_client_order_id: None,
            side,
            quantity: None,
            above_type,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    ///	Quantity for both legs of the order list.
    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /// Arbitrary unique ID among open orders for the above leg order.
    /// Automatically generated if not sent.
    pub fn above_client_order_id(mut self, above_client_order_id: &str) -> Self {
        self.above_client_order_id = Some(above_client_order_id.to_owned());
        self
    }

    pub fn above_iceberg_qty(mut self, above_iceberg_qty: i64) -> Self {
        self.above_iceberg_qty = Some(above_iceberg_qty);
        self
    }

    pub fn above_price(mut self, above_price: f64) -> Self {
        self.above_price = Some(above_price);
        self
    }

    pub fn above_stop_price(mut self, above_stop_price: f64) -> Self {
        self.above_stop_price = Some(above_stop_price);
        self
    }

    pub fn above_trailing_delta(mut self, above_trailing_delta: f64) -> Self {
        self.above_trailing_delta = Some(above_trailing_delta);
        self
    }

    pub fn above_time_in_force(mut self, above_time_in_force: TimeInForce) -> Self {
        self.above_time_in_force = Some(above_time_in_force);
        self
    }

    /// Arbitrary numeric value identifying the above leg order within an order
    /// strategy.
    pub fn above_strategy_id(mut self, above_strategy_id: i64) -> Self {
        self.above_strategy_id = Some(above_strategy_id);
        self
    }

    /// Arbitrary numeric value identifying the above leg order strategy. Values
    /// smaller than 1000000 are reserved and cannot be used.
    pub fn above_strategy_type(mut self, above_strategy_type: i64) -> Self {
        self.above_strategy_type = Some(above_strategy_type);
        self
    }

    pub fn below_client_order_id(mut self, below_client_order_id: &str) -> Self {
        self.below_client_order_id = Some(below_client_order_id.to_owned());
        self
    }

    pub fn below_iceberg_qty(mut self, below_iceberg_qty: i64) -> Self {
        self.below_iceberg_qty = Some(below_iceberg_qty);
        self
    }

    pub fn below_price(mut self, below_price: f64) -> Self {
        self.below_price = Some(below_price);
        self
    }

    pub fn below_stop_price(mut self, below_stop_price: f64) -> Self {
        self.below_stop_price = Some(below_stop_price);
        self
    }

    pub fn below_trailing_delta(mut self, below_trailing_delta: f64) -> Self {
        self.below_trailing_delta = Some(below_trailing_delta);
        self
    }

    pub fn below_time_in_force(mut self, below_time_in_force: TimeInForce) -> Self {
        self.below_time_in_force = Some(below_time_in_force);
        self
    }

    /// Arbitrary numeric value identifying the below leg order within an order
    /// strategy.
    pub fn below_strategy_id(mut self, below_strategy_id: i64) -> Self {
        self.below_strategy_id = Some(below_strategy_id);
        self
    }

    ///	Arbitrary numeric value identifying the below leg order strategy. Values
    /// smaller than 1000000 are reserved and cannot be used.
    pub fn below_strategy_type(mut self, below_strategy_type: i64) -> Self {
        self.below_strategy_type = Some(below_strategy_type);
        self
    }

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types
    /// default to FULL, all other orders default to ACK.
    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderRespType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    /// The allowed enums is dependent on what is configured on the symbol. The
    /// possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH,
    /// NONE.
    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: SelfTradePreventionMode,
    ) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

pub type NewOrderListOcoResponse = OrderListResult;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResult {
    pub order_list_id: i64,
    pub contingency_type: ContingencyType,
    pub list_status_type: OrderListStatus,
    pub list_order_status: OrderListOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: i64,
    pub symbol: String,
    #[serde(default)]
    pub orders: Vec<OrderListItem>,
    #[serde(default)]
    pub order_reports: Vec<OrderListReport>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListItem {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListReport {
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
    pub stop_price: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub transaction_time: i64,
}
