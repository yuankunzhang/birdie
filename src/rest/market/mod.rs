//! Market Data endpoints
mod aggregate_trades_list;
mod kline_data;
mod old_trade_lookup;
mod order_book;
mod recent_trades_list;

pub use aggregate_trades_list::*;
pub use kline_data::*;
pub use old_trade_lookup::*;
pub use order_book::*;
pub use recent_trades_list::*;

use super::{route, RestClient};

pub struct Handler<'r> {
    client: &'r RestClient,
}

impl<'r> Handler<'r> {
    pub fn new(client: &'r RestClient) -> Self {
        Handler { client }
    }

    route!(order_book, OrderBookEndpoint);
    route!(recent_trades_list, RecentTradesListEndpoint);
    route!(old_trade_lookup, OldTradeLookupEndpoint);
    route!(aggregate_trades_list, AggregateTradesListEndpoint);
    route!(kline_data, KlineDataEndpoint);
}
