mod close_isolated_margin_user_data_stream;
mod close_margin_user_data_stream;
mod keepalive_isolated_margin_user_data_stream;
mod keepalive_margin_user_data_stream;
mod start_isolated_margin_user_data_stream;
mod start_margin_user_data_stream;

pub use close_isolated_margin_user_data_stream::*;
pub use close_margin_user_data_stream::*;
pub use keepalive_isolated_margin_user_data_stream::*;
pub use keepalive_margin_user_data_stream::*;
use serde::Deserialize;
pub use start_isolated_margin_user_data_stream::*;
pub use start_margin_user_data_stream::*;

use crate::{
    rest_api::{route, RestApiClient},
    spot::user_data_stream::{
        BalanceUpdate, ExecutionReport, ListStatus, ListenKeyExpired, OutboundAccountPosition,
    },
    web_socket_stream::Payload,
};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(start_margin_data_stream, StartMarginUserDataStreamEndpoint);
    route!(
        keepalive_margin_data_stream,
        KeepaliveMarginUserDataStreamEndpoint
    );
    route!(close_margin_data_stream, CloseMarginUserDataStreamEndpoint);
    route!(
        start_isolated_margin_data_stream,
        StartIsolatedMarginUserDataStreamEndpoint
    );
    route!(
        keepalive_isolated_margin_data_stream,
        KeepaliveIsolatedMarginUserDataStreamEndpoint
    );
    route!(
        close_isolated_margin_data_stream,
        CloseIsolatedMarginUserDataStreamEndpoint
    );
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "e")]
pub enum TradeUserDataStreamPayload {
    OutboundAccountPosition(OutboundAccountPosition),
    BalanceUpdate(BalanceUpdate),
    ExecutionReport(ExecutionReport),
    ListStatus(ListStatus),
    ListenKeyExpired(ListenKeyExpired),
}

impl Payload for TradeUserDataStreamPayload {}
