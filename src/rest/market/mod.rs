//! Market Data endpoints
mod check_server_time;
mod exchange_info;
mod test_connectivity;

pub use check_server_time::*;
pub use exchange_info::*;
pub use test_connectivity::*;

use super::RestClient;

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    pub fn test_connectivity(&self) -> TestConnectivityEndpoint {
        TestConnectivityEndpoint::new(self.client)
    }

    pub fn check_server_time(&self) -> CheckServerTimeEndpoint {
        CheckServerTimeEndpoint::new(self.client)
    }

    pub fn exchange_info(&self) -> ExchangeInfoEndpoint {
        ExchangeInfoEndpoint::new(self.client)
    }
}
