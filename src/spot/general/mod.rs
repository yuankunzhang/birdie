//! General endpoints
//!
//! - [`test_connectivity`](TestConnectivityEndpoint)
//! - [`check_server_time`](CheckServerTimeEndpoint)
//! - [`exchange_info`](ExchangeInfoEndpoint)
mod check_server_time;
mod exchange_info;
mod test_connectivity;

pub use check_server_time::*;
pub use exchange_info::*;
pub use test_connectivity::*;

use crate::{
    rest_api::{route, RestApiClient},
    web_socket_api::{ws_route, WebSocketApiClient},
};

pub struct RestApiHandler<'r> {
    client: &'r RestApiClient,
}

impl<'r> RestApiHandler<'r> {
    pub fn new(client: &'r RestApiClient) -> Self {
        RestApiHandler { client }
    }

    route!(test_connectivity, TestConnectivityEndpoint);
    route!(check_server_time, CheckServerTimeEndpoint);
    route!(exchange_info, ExchangeInfoEndpoint);
}

pub struct WebSocketApiHandler<'w> {
    client: &'w WebSocketApiClient,
}

impl<'w> WebSocketApiHandler<'w> {
    pub fn new(client: &'w WebSocketApiClient) -> Self {
        WebSocketApiHandler { client }
    }

    ws_route!(test_connectivity, TestConnectivityWebSocket);
    ws_route!(check_server_time, CheckServerTimeWebSocket);
    ws_route!(exchange_info, ExchangeInfoWebSocket);
}
