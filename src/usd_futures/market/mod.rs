mod aggregate_trades_list;
mod check_server_time;
mod composite_index_symbol_information;
mod continuous_contract_klines;
mod exchange_info;
mod get_funding_info;
mod get_funding_rate_history;
mod historical_blvt_nav_klines;
mod index_price_klines;
mod klines;
mod long_short_ratio;
mod mark_price;
mod mark_price_klines;
mod multi_assets_mode_asset_index;
mod old_trade_lookup;
mod open_interest;
mod open_interest_statistics;
mod order_book;
mod premium_index_klines;
mod query_delivery_price;
mod query_index_price_constituents;
mod recent_trades_list;
mod symbol_order_book_ticker;
mod symbol_order_book_ticker_v2;
mod symbol_price_ticker;
mod taker_buy_sell_volume;
mod test_connectivity;
mod ticker_24hr;
mod top_trader_long_short_account_ratio;
mod top_trader_long_short_position_ratio;

pub use aggregate_trades_list::*;
pub use check_server_time::*;
pub use composite_index_symbol_information::*;
pub use continuous_contract_klines::*;
pub use exchange_info::*;
pub use get_funding_info::*;
pub use get_funding_rate_history::*;
pub use historical_blvt_nav_klines::*;
pub use index_price_klines::*;
pub use klines::*;
pub use long_short_ratio::*;
pub use mark_price::*;
pub use mark_price_klines::*;
pub use multi_assets_mode_asset_index::*;
pub use old_trade_lookup::*;
pub use open_interest::*;
pub use open_interest_statistics::*;
pub use order_book::*;
pub use premium_index_klines::*;
pub use query_delivery_price::*;
pub use query_index_price_constituents::*;
pub use recent_trades_list::*;
pub use symbol_order_book_ticker::*;
pub use symbol_order_book_ticker_v2::*;
pub use symbol_price_ticker::*;
pub use taker_buy_sell_volume::*;
pub use test_connectivity::*;
pub use ticker_24hr::*;
pub use top_trader_long_short_account_ratio::*;
pub use top_trader_long_short_position_ratio::*;

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

    route!(check_server_time, CheckServerTimeEndpoint);
    route!(
        composite_index_symbol_information,
        CompositeIndexSymbolInformationEndpoint
    );
    route!(aggregate_trades_list, AggregateTradesListEndpoint);
    route!(continuous_contract_klines, ContinuousContractKlinesEndpoint);
    route!(exchange_info, ExchangeInfoEndpoint);
    route!(get_funding_info, GetFundingInfoEndpoint);
    route!(get_funding_rate_history, GetFundingRateHistoryEndpoint);
    route!(historical_blvt_nav_klines, HistoricalBlvtNavKlinesEndpoint);
    route!(index_price_klines, IndexPriceKlinesEndpoint);
    route!(klines, KlinesEndpoint);
    route!(long_short_ratio, LongShortRatioEndpoint);
    route!(mark_price, MarkPriceEndpoint);
    route!(mark_price_klines, MarkPriceKlinesEndpoint);
    route!(
        multi_assets_mode_asset_index,
        MultiAssetsModeAssetIndexEndpoint
    );
    route!(old_trade_lookup, OldTradeLookupEndpoint);
    route!(open_interest, OpenInterestEndpoint);
    route!(open_interest_statistics, OpenInterestStatisticsEndpoint);
    route!(order_book, OrderBookEndpoint);
    route!(premium_index_klines, PremiumIndexKlinesEndpoint);
    route!(query_delivery_price, QueryDeliveryPriceEndpoint);
    route!(
        query_index_price_constituents,
        QueryIndexPriceConstituentsEndpoint
    );
    route!(recent_trades_list, RecentTradesListEndpoint);
    route!(symbol_order_book_ticker, SymbolOrderBookTickerEndpoint);
    route!(symbol_order_book_ticker_v2, SymbolOrderBookTickerV2Endpoint);
    route!(symbol_price_ticker, SymbolPriceTickerEndpoint);
    route!(taker_buy_sell_volume, TakerBuySellVolumeEndpoint);
    route!(test_connectivity, TestConnectivityEndpoint);
    route!(ticker_24hr, Ticker24hrEndpoint);
    route!(
        top_trader_long_short_account_ratio,
        TopTraderLongShortAccountRatioEndpoint
    );
    route!(
        top_trader_long_short_position_ratio,
        TopTraderLongShortPositionRatioEndpoint
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
    ws_route!(symbol_order_book_ticker, SymbolOrderBookTickerWebSocket);
    ws_route!(symbol_price_ticker, SymbolPriceTickerWebSocket);
}
