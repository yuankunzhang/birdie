//! Market Data endpoints
mod aggregate_trades_list;
mod current_average_price;
mod kline_data;
mod old_trade_lookup;
mod order_book;
mod recent_trades_list;
mod ui_klines;

pub use aggregate_trades_list::*;
pub use current_average_price::*;
pub use kline_data::*;
pub use old_trade_lookup::*;
pub use order_book::*;
pub use recent_trades_list::*;
pub use ui_klines::*;

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
    route!(ui_klines, UiKlinesEndpoint);
    route!(current_average_price, CurrentAveragePriceEndpoint);
}
