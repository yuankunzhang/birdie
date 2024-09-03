use jiff::Timestamp;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{OrderSide, OrderType, PreventionMode, TimeInForce},
    rest::{endpoint, SecurityType},
};

endpoint!(
    "/api/v3/order",
    Method::POST,
    SecurityType::Trade,
    NewOrderEndpoint,
    NewOrderParams,
    NewOrderResponse
);

/// Send in a new order.
///
/// - Weight: 1
/// - Data Source: Matching Engine
pub struct NewOrderEndpoint<'r> {
    client: &'r crate::rest::RestClient,
}

impl<'r> NewOrderEndpoint<'r> {
    pub fn new(client: &'r crate::rest::RestClient) -> Self {
        Self { client }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderParams {
    symbol: String,
    side: OrderSide,
    r#type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<f64>,
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
    new_order_resp_type: Option<OrderResponseResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_mode: Option<PreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponseResult {
    /// Example: `"BTCUSDT"`
    pub symbol: String,
    /// Example: `28`
    pub order_id: i64,
    /// Example: `-1`
    pub order_list_id: i64,
    /// Example: `"6gCrw2kRUAF9CvJDGP16IP"`
    pub client_order_id: String,
    /// Example: `1507725176595`
    pub transact_time: i64,
    /// Example: `"0.00000000"`
    pub price: String,
    /// Example: `"10.00000000"`
    pub orig_qty: String,
    /// Example: `"10.00000000"`
    pub executed_qty: String,
    /// Example: `"10.00000000"`
    pub cummulative_quote_qty: String,
    /// Example: `"FILLED"`
    pub status: String,
    /// Example: `"GTC"`
    pub time_in_force: String,
    /// Example: `"MARKET"`
    pub r#type: String,
    /// Example: `"SELL"`
    pub side: String,
    /// Example: `1`
    pub strategy_id: Option<i64>,
    /// Example: `1000000`
    pub strategy_type: Option<i64>,
    /// Example: `1507725176595`
    pub working_time: i64,
    /// Example: `"NONE"`
    pub self_trade_prevention_mode: String,
}

impl NewOrderParams {
    pub fn new(symbol: &str, side: OrderSide, r#type: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
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

    pub fn quote_order_qty(mut self, quote_order_qty: f64) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// A unique id among open orders. Automatically generated if not sent.
    /// Orders with the same `newClientOrderID` can be accepted only when the
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

    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and
    /// TAKE_PROFIT_LIMIT orders.
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /// Used with STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and
    /// TAKE_PROFIT_LIMIT orders.
    pub fn trailing_delta(mut self, trailing_delta: f64) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    /// Used with LIMIT, STOP_LOSS_LIMIT, and TAKE_PROFIT_LIMIT to create an
    /// iceberg order.
    pub fn iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
        self
    }

    /// Set the response JSON. ACK, RESULT, or FULL; MARKET and LIMIT order types
    /// default to FULL, all other orders default to ACK.
    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseResult) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    /// The allowed enums is dependent on what is configured on the symbol. The
    /// possible supported values are EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH,
    /// NONE.
    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: PreventionMode,
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
#[serde(untagged)]
pub enum NewOrderResponse {
    Ack(Box<NewOrderAck>),
    Result(Box<NewOrderResult>),
    Full(Box<OrderFull>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderAck {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    #[serde(flatten)]
    pub conditional: ConditionalFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResult {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub working_type: i64,
    pub self_trade_prevention_mode: String,
    #[serde(flatten)]
    pub conditional: ConditionalFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFull {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub working_type: i64,
    pub self_trade_prevention_mode: String,
    pub fills: Vec<Fill>,
    #[serde(flatten)]
    pub conditional: ConditionalFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub trade_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalFields {
    /// Quantity for the iceberg order. Appears only if the parameter
    /// `iceberg_qty` was sent in the request.
    pub iceberg_qty: Option<String>,
    /// When used in combination with `symbol`, can be used to query a prevented
    /// match. Appears only if the order exired due to STP.
    pub prevented_match_id: Option<i64>,
    /// Order quantity that expired due to STP. Appears only if the order
    /// expired due to STP.
    pub prevented_quantity: Option<String>,
    /// Price when the algorithmic order will be triggered. Appears for
    /// `StopLoss`, `TakeProfit`, `StopLossLimit` and `TakeProfitLimit` orders.
    pub stop_price: Option<String>,
    /// Can be used to label an order that's part of an order strategy. Appears
    /// if the parameter was populated in the request.
    pub strategy_id: Option<i64>,
    /// Can be used to label an order that is using an order strategy. Appears if
    /// the parameter was populated in the request.
    pub strategy_type: Option<i64>,
    /// Delta price change required before order activation. Appears for Trailing
    /// Stop Orders.
    pub trailing_delta: Option<i64>,
    /// Time when the trailing order is now active and tracking price changes.
    /// Appears only for Trailing Stop Orders.
    pub trailing_time: Option<i64>,
    /// Field that determines whether order used SOR. Appears when placing orders
    /// using SOR.
    pub used_sor: Option<bool>,
    /// Field that determines whether the order is being filled by the SOR or by
    /// the order book the order was submitted to. Appears when placing orders
    /// using SOR
    pub working_floor: Option<String>,
}
