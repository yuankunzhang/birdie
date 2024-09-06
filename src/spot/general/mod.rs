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
    web_socket_api::{target, WebSocketApiClient},
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

    target!(test_connectivity, TestConnectivityWebSocket);
    target!(check_server_time, CheckServerTimeWebSocket);
    target!(exchange_info, ExchangeInfoWebSocket);
}
