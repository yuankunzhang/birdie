mod close_user_data_stream;
mod keepalive_user_data_stream;
mod start_user_data_stream;

pub use close_user_data_stream::*;
pub use keepalive_user_data_stream::*;
use serde::Deserialize;
pub use start_user_data_stream::*;

use crate::{
    enums::{
        ContingencyType, OrderListOrderStatus, OrderListStatus, OrderSide, OrderType,
        SelfTradePreventionMode, TimeInForce,
    },
    rest_api::{route, RestApiClient},
    web_socket_stream::Payload,
};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(start_user_data_stream, StartUserDataStreamEndpoint);
    route!(keepalive_user_data_stream, KeepaliveUserDataStreamEndpoint);
    route!(close_user_data_stream, CloseUserDataStreamEndpoint);
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "e")]
pub enum UserDataStreamPayload {
    OutboundAccountPosition(OutboundAccountPosition),
    BalanceUpdate(BalanceUpdate),
    ExecutionReport(ExecutionReport),
    ListStatus(ListStatus),
    ListenKeyExpired(ListenKeyExpired),
}

impl Payload for UserDataStreamPayload {}

#[derive(Clone, Debug, Deserialize)]
pub struct OutboundAccountPosition {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "u")]
    pub last_account_update: i64,
    #[serde(rename = "B")]
    pub balances: Vec<BalanceData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BalanceData {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: String,
    #[serde(rename = "l")]
    pub locked: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BalanceUpdate {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    pub balance_delta: String,
    #[serde(rename = "T")]
    pub clear_time: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionReport {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub r#type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "P")]
    pub stop_price: String,
    #[serde(rename = "F")]
    pub iceberg_quantity: String,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "C")]
    pub orig_client_order_id: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "r")]
    pub reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "l")]
    pub last_executed_quantity: String,
    #[serde(rename = "z")]
    pub cummulative_filled_quantity: String,
    #[serde(rename = "L")]
    pub last_executed_price: String,
    #[serde(rename = "n")]
    pub commission_amount: String,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "I")]
    pub ignore1: i64,
    #[serde(rename = "w")]
    pub is_on_order_book: bool,
    #[serde(rename = "m")]
    pub is_maker_side: bool,
    #[serde(rename = "M")]
    pub ignore2: bool,
    #[serde(rename = "O")]
    pub order_creation_time: i64,
    #[serde(rename = "Z")]
    pub cumulative_quote_asset_quantity: String,
    #[serde(rename = "Y")]
    pub last_quote_asset_quantity: String,
    #[serde(rename = "Q")]
    pub quote_order_quantity: String,
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    #[serde(flatten)]
    pub conditional: ConditionalData,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConditionalData {
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "d")]
    pub trailing_delta: Option<i64>,
    #[serde(rename = "d")]
    pub trailing_time: Option<i64>,
    #[serde(rename = "j")]
    pub strategy_id: Option<i64>,
    #[serde(rename = "J")]
    pub strategy_type: Option<i64>,
    #[serde(rename = "v")]
    pub prevented_match_id: Option<i64>,
    #[serde(rename = "A")]
    pub prevented_quantity: Option<String>,
    #[serde(rename = "B")]
    pub last_prevented_quantity: Option<String>,
    #[serde(rename = "u")]
    pub trade_group_id: Option<i64>,
    #[serde(rename = "U")]
    pub counter_order_id: Option<i64>,
    #[serde(rename = "Cs")]
    pub counter_symbol: Option<String>,
    #[serde(rename = "pl")]
    pub prevented_execution_quantity: Option<String>,
    #[serde(rename = "pL")]
    pub prevented_execution_price: Option<String>,
    #[serde(rename = "pY")]
    pub prevented_execution_quote_qty: Option<String>,
    #[serde(rename = "W")]
    pub working_time: Option<i64>,
    #[serde(rename = "b")]
    pub match_type: Option<String>,
    #[serde(rename = "a")]
    pub allocation_id: Option<i64>,
    #[serde(rename = "k")]
    pub working_floor: Option<String>,
    #[serde(rename = "uS")]
    pub used_sor: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListStatus {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "c")]
    pub contingency_type: ContingencyType,
    #[serde(rename = "l")]
    pub list_status_type: OrderListStatus,
    #[serde(rename = "L")]
    pub list_order_status: OrderListOrderStatus,
    #[serde(rename = "r")]
    pub list_reject_reason: String,
    #[serde(rename = "C")]
    pub list_client_order_id: String,
    #[serde(rename = "O")]
    pub transaction_time: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListStatusObject {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ListenKeyExpired {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}
