use birdie::{
    enums::KlineInterval,
    rest_api::Endpoint,
    spot::market::{
        AggregateTradesListParams, CurrentAveragePriceParams, KlinesParams, OldTradeLookupParams,
        OrderBookParams, RecentTradesListParams, RollingWindowPriceChangeParams,
        SymbolOrderBookTickerParams, SymbolPriceTickerParams, Ticker24hrParams,
        TradingDayTickerParams, UiKlinesParams,
    },
};

mod common;

#[tokio::test]
async fn rest_order_book() {
    let client = common::setup_rest_api_client();
    let params = OrderBookParams::new("BTCUSDT").limit(10);
    let resp = client.market().order_book().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_recent_trades_list() {
    let client = common::setup_rest_api_client();
    let params = RecentTradesListParams::new("BTCUSDT").limit(10);
    let resp = client.market().recent_trades_list().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_old_trade_lookup() {
    let client = common::setup_rest_api_client();
    let params = OldTradeLookupParams::new("BTCUSDT").limit(10);
    let resp = client.market().old_trade_lookup().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_aggregate_trades_list() {
    let client = common::setup_rest_api_client();
    let params = AggregateTradesListParams::new("BTCUSDT").limit(10);
    let resp = client
        .market()
        .aggregate_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_kline_data() {
    let client = common::setup_rest_api_client();
    let params = KlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = client.market().kline_data().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_ui_kline() {
    let client = common::setup_rest_api_client();
    let params = UiKlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = client.market().ui_klines().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_current_average_price() {
    let client = common::setup_rest_api_client();
    let params = CurrentAveragePriceParams::new("BTCUSDT");
    let resp = client
        .market()
        .current_average_price()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_ticker_24hr() {
    let client = common::setup_rest_api_client();
    let params = Ticker24hrParams::new().symbol("BTCUSDT");
    let resp = client.market().ticker_24hr().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_trading_day_ticker() {
    let client = common::setup_rest_api_client();
    let params = TradingDayTickerParams::new().symbol("BTCUSDT");
    let resp = client.market().trading_day_ticker().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_symbol_price_ticker() {
    let client = common::setup_rest_api_client();
    let params = SymbolPriceTickerParams::new().symbol("BTCUSDT");
    let resp = client.market().symbol_price_ticker().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_symbol_order_book_ticker() {
    let client = common::setup_rest_api_client();
    let params = SymbolOrderBookTickerParams::new().symbol("BTCUSDT");
    let resp = client
        .market()
        .symbol_order_book_ticker()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_rolling_window_price_change() {
    let client = common::setup_rest_api_client();

    let params = RollingWindowPriceChangeParams::new().symbol("BTCUSDT");
    let resp = client
        .market()
        .rolling_window_price_change()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
