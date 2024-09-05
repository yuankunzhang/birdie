use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        NewOrderRespType, OrderSide, OrderStatus, OrderType, SelfTradePreventionMode, TimeInForce,
    },
    rest_api::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/sor/order",
    Method::POST,
    SecurityType::Trade,
    NewOrderUsingSorEndpoint,
    NewOrderUsingSorParams,
    NewOrderUsingSorResponse
);

/// Places an order using smart order routing (SOR).
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderUsingSorEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> NewOrderUsingSorEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderUsingSorParams {
    symbol: String,
    side: OrderSide,
    r#type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<NewOrderRespType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl NewOrderUsingSorParams {
    pub fn new(symbol: &str, side: OrderSide, r#type: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type,
            time_in_force: None,
            quantity: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: Timestamp::now().as_millisecond(),
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// A unique id among open orders. Automatically generated if not sent.
    /// Orders with the same `new_client_order_id` can be accepted only when the
    /// previous one is filled, otherwise the order will be rejected.
    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn strategy_id(mut self, strategy_id: i64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    /// The value cannot be less than 1000000.
    pub fn strategy_type(mut self, strategy_type: &str) -> Self {
        self.strategy_type = Some(strategy_type.to_owned());
        self
    }

    /// Used with `LIMIT` to create an iceberg order.
    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
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

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderUsingSorResponse {
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
    pub side: OrderSide,
    pub working_time: i64,
    pub fills: Vec<SorOrderFill>,
    pub working_floor: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub used_sor: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SorOrderFill {
    pub match_type: String,
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub trade_id: i64,
    pub alloc_id: i64,
}
