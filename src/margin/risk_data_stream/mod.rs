mod close_user_data_stream;
mod keepalive_user_data_stream;
mod start_user_data_stream;

pub use close_user_data_stream::*;
pub use keepalive_user_data_stream::*;
use serde::Deserialize;
pub use start_user_data_stream::*;

use crate::{
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
pub enum RiskDataStreamPayload {
    MarginCall(MarginCall),
    LiabilityUpdate(LiabilityUpdate),
}

impl Payload for RiskDataStreamPayload {}

#[derive(Clone, Debug, Deserialize)]
pub struct MarginCall {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "l")]
    pub margin_level: String,
    #[serde(rename = "s")]
    pub margin_call_status: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LiabilityUpdate {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "t")]
    pub liability_update_type: String,
    #[serde(rename = "p")]
    pub principle_quantity: String,
    #[serde(rename = "i")]
    pub interest_quantity: String,
}
