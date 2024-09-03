//! General endpoints
mod check_server_time;
mod exchange_info;
mod test_connectivity;

pub use check_server_time::*;
pub use exchange_info::*;
pub use test_connectivity::*;

use super::{route, RestClient};

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    route!(test_connectivity, TestConnectivityEndpoint);
    route!(check_server_time, CheckServerTimeEndpoint);
    route!(exchange_info, ExchangeInfoEndpoint);
}
