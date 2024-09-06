use birdie::{
    enums::KlineInterval,
    rest_api::Endpoint,
    spot::market::{
        AggregateTradesListParams, CurrentAveragePriceParams, KlineDataParams,
        OldTradeLookupParams, OrderBookParams, RecentTradesListParams,
        RollingWindowPriceChangeParams, SymbolOrderBookTickerParams, SymbolPriceTickerParams,
        Ticker24hrParams, TradingDayTickerParams, UiKlinesParams,
    },
};

mod common;

#[tokio::test]
async fn order_book() {
    let birdie = common::setup();
    let params = OrderBookParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest_api()
        .market()
        .order_book()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn recent_trades_list() {
    let birdie = common::setup();
    let params = RecentTradesListParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest_api()
        .market()
        .recent_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn old_trade_lookup() {
    let birdie = common::setup();
    let params = OldTradeLookupParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest_api()
        .market()
        .old_trade_lookup()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn aggregate_trades_list() {
    let birdie = common::setup();
    let params = AggregateTradesListParams::new("BTCUSDT").limit(10);
    let resp = birdie
        .rest_api()
        .market()
        .aggregate_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn kline_data() {
    let birdie = common::setup();
    let params = KlineDataParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = birdie
        .rest_api()
        .market()
        .kline_data()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ui_kline() {
    let birdie = common::setup();
    let params = UiKlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = birdie.rest_api().market().ui_klines().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn current_average_price() {
    let birdie = common::setup();
    let params = CurrentAveragePriceParams::new("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .current_average_price()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ticker_24hr() {
    let birdie = common::setup();
    let params = Ticker24hrParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .ticker_24hr()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn trading_day_ticker() {
    let birdie = common::setup();
    let params = TradingDayTickerParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .trading_day_ticker()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn symbol_price_ticker() {
    let birdie = common::setup();
    let params = SymbolPriceTickerParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .symbol_price_ticker()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn symbol_order_book_ticker() {
    let birdie = common::setup();
    let params = SymbolOrderBookTickerParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .symbol_order_book_ticker()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rolling_window_price_change() {
    let birdie = common::setup();

    // price too low
    let params = RollingWindowPriceChangeParams::new().symbol("BTCUSDT");
    let resp = birdie
        .rest_api()
        .market()
        .rolling_window_price_change()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
