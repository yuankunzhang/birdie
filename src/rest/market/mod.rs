//! Market Data endpoints
mod check_server_time;
mod exchange_info;
mod order_book;
mod recent_trades_list;
mod test_connectivity;

pub use check_server_time::*;
pub use exchange_info::*;
pub use order_book::*;
pub use recent_trades_list::*;
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
    route!(order_book, OrderBookEndpoint);
    route!(recent_trades_list, RecentTradesListEndpoint);
}
