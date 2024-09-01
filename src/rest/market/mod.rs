//! Market Data endpoints
mod check_server_time;
mod exchange_info;
mod order_book;
mod test_connectivity;

pub use check_server_time::*;
pub use exchange_info::*;
pub use order_book::*;
pub use test_connectivity::*;

use super::RestClient;

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    /// Test connectivity to the Rest API.
    ///
    /// Weight(IP): 1
    pub fn test_connectivity(&self) -> TestConnectivityEndpoint {
        TestConnectivityEndpoint::new(self.client)
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight(IP): 1
    pub fn check_server_time(&self) -> CheckServerTimeEndpoint {
        CheckServerTimeEndpoint::new(self.client)
    }

    /// Current exchange trading rules and symbol information.
    ///
    /// Weight(IP): 20
    pub fn exchange_info(&self) -> ExchangeInfoEndpoint {
        ExchangeInfoEndpoint::new(self.client)
    }

    /// Get order book.
    ///
    /// Weight(IP): Adjusted based on the limit.
    ///
    /// | Limit     | Weight |
    /// |-----------|--------|
    /// | 1-100     | 5      |
    /// | 101-500   | 25     |
    /// | 501-1000  | 50     |
    /// | 1001-5000 | 250    |
    pub fn order_book(&self) -> OrderBookEndpoint {
        OrderBookEndpoint::new(self.client)
    }
}
