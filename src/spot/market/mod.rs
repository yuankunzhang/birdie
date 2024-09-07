//! Market Data endpoints
//!
//! - [`order_book`](OrderBookEndpoint)
//! - [`recent_trades_list`](RecentTradesListEndpoint)
//! - [`old_trade_lookup`](OldTradeLookupEndpoint)
//! - [`aggregate_trades_list`](AggregateTradesListEndpoint)
//! - [`kline_data`](KlineDataEndpoint)
//! - [`ui_klines`](UiKlinesEndpoint)
//! - [`current_average_price`](CurrentAveragePriceEndpoint)
//! - [`ticker_24hr`](Ticker24hrEndpoint)
//! - [`trading_day_ticker`](TradingDayTickerEndpoint)
//! - [`symbol_price_ticker`](SymbolPriceTickerEndpoint)
//! - [`symbol_order_book_ticker`](SymbolOrderBookTickerEndpoint)
//! - [`rolling_window_price_change`](RollingWindowPriceChangeEndpoint)
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

pub struct WebSocketApiHandler<'w> {
    client: &'w WebSocketApiClient,
}

impl<'w> WebSocketApiHandler<'w> {
    pub fn new(client: &'w WebSocketApiClient) -> Self {
        WebSocketApiHandler { client }
    }

    ws_route!(order_book, OrderBookWebSocket);
    ws_route!(recent_trades_list, RecentTradesListWebSocket);
    ws_route!(old_trade_lookup, OldTradeLookupWebSocket);
    ws_route!(aggregate_trades_list, AggregateTradesListWebSocket);
    ws_route!(kline_data, KlineDataWebSocket);
    ws_route!(ui_klines, UiKlinesWebSocket);
    ws_route!(current_average_price, CurrentAveragePriceWebSocket);
    ws_route!(ticker_24hr, Ticker24hrWebSocket);
    ws_route!(trading_day_ticker, TradingDayTickerWebSocket);
    ws_route!(symbol_price_ticker, SymbolPriceTickerWebSocket);
    ws_route!(symbol_order_book_ticker, SymbolOrderBookTickerWebSocket);
    ws_route!(
        rolling_window_price_change,
        RollingWindowPriceChangeWebSocket
    );
}
