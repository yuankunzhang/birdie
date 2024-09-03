//! Market Data endpoints
mod aggregate_trades_list;
mod current_average_price;
mod kline_data;
mod old_trade_lookup;
mod order_book;
mod recent_trades_list;
mod rolling_window_price_change;
mod symbol_order_book_ticker;
mod symbol_price_ticker;
mod ticker_24hr;
mod trading_day_ticker;
mod ui_klines;

pub use aggregate_trades_list::*;
pub use current_average_price::*;
pub use kline_data::*;
pub use old_trade_lookup::*;
pub use order_book::*;
pub use recent_trades_list::*;
pub use rolling_window_price_change::*;
pub use symbol_order_book_ticker::*;
pub use symbol_price_ticker::*;
pub use ticker_24hr::*;
pub use trading_day_ticker::*;
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
    route!(ticker_24hr, Ticker24hrEndpoint);
    route!(trading_day_ticker, TradingDayTickerEndpoint);
    route!(symbol_price_ticker, SymbolPriceTickerEndpoint);
    route!(symbol_order_book_ticker, SymbolOrderBookTickerEndpoint);
    route!(
        rolling_window_price_change,
        RollingWindowPriceChangeEndpoint
    );
}
