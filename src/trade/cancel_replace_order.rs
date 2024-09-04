use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{
        CancelReplaceMode, CancelRestriction, NewOrderRespType, OrderRateLimitExceededMode,
        OrderSide, OrderType, SelfTradePreventionMode, TimeInForce,
    },
    errors::BinanceError,
    rest_api::{endpoint, SecurityType},
};

use super::{CancelOrderResult, NewOrderResult};

endpoint!(
    "/api/v3/cancelReplace",
    Method::POST,
    SecurityType::Trade,
    CancelReplaceOrderEndpoint,
    CancelReplaceOrderParams,
    CancelReplaceOrderResponse
);

/// Cancels an existing order and places a new order on the same symbol.
///
/// Filters and Order Count are evaluated before the processing of the
/// cancellation and order placement occurs.
///
/// A new order that was not attempted (i.e. when newOrderResult:
/// NOT_ATTEMPTED ), will still increase the order count by 1.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct CancelReplaceOrderEndpoint<'r> {
    client: &'r crate::rest_api::RestApiClient,
}

impl<'r> CancelReplaceOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest_api::RestApiClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderParams {
    symbol: String,
    side: OrderSide,
    r#type: OrderType,
    cancel_replace_mode: CancelReplaceMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_order_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_order_resp_type: Option<NewOrderRespType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_restrictions: Option<CancelRestriction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_rate_limit_exceeded_mode: Option<OrderRateLimitExceededMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

impl CancelReplaceOrderParams {
    pub fn new(
        symbol: &str,
        side: OrderSide,
        r#type: OrderType,
        cancel_replace_mode: CancelReplaceMode,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type,
            cancel_replace_mode,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            cancel_new_client_order_id: None,
            cancel_orig_client_order_id: None,
            cancel_order_id: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
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

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn cancel_new_client_order_id(mut self, cancel_new_client_order_id: &str) -> Self {
        self.cancel_new_client_order_id = Some(cancel_new_client_order_id.to_owned());
        self
    }

    pub fn cancel_orig_client_order_id(mut self, cancel_orig_client_order_id: &str) -> Self {
        self.cancel_orig_client_order_id = Some(cancel_orig_client_order_id.to_owned());
        self
    }

    pub fn cancel_order_id(mut self, cancel_order_id: i64) -> Self {
        self.cancel_order_id = Some(cancel_order_id);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn strategy_id(mut self, strategy_id: i64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    pub fn strategy_type(mut self, strategy_type: &str) -> Self {
        self.strategy_type = Some(strategy_type.to_owned());
        self
    }

    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn trailing_delta(mut self, trailing_delta: f64) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

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

    pub fn cancel_restrictions(mut self, cancel_restrictions: CancelRestriction) -> Self {
        self.cancel_restrictions = Some(cancel_restrictions);
        self
    }

    pub fn order_rate_limit_exceeded_mode(
        mut self,
        order_rate_limit_exceeded_mode: OrderRateLimitExceededMode,
    ) -> Self {
        self.order_rate_limit_exceeded_mode = Some(order_rate_limit_exceeded_mode);
        self
    }

    /// The value cannot be greater than 60000.
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CancelReplaceOrderResponse {
    Success(Success),
    NotSuccess(NotSuccess),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Success {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: CancelResult,
    pub new_order_response: NewResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotSuccess {
    #[serde(flatten)]
    pub error: BinanceError,
    pub data: NotSuccessData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotSuccessData {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: CancelResult,
    pub new_order_response: NewResult,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CancelResult {
    Success(CancelOrderResult),
    Failure(BinanceError),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NewResult {
    Success(NewOrderResult),
    Failure(BinanceError),
}
