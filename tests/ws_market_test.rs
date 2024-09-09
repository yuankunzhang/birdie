use birdie::{
    enums::KlineInterval,
    spot::market::{
        AggregateTradesListParams, CurrentAveragePriceParams, KlinesParams,
        OldTradeLookupParams, OrderBookParams, RecentTradesListParams,
        RollingWindowPriceChangeParams, SymbolOrderBookTickerParams, SymbolPriceTickerParams,
        Ticker24hrParams, TradingDayTickerParams, UiKlinesParams,
    },
    web_socket::ConnectionStatus,
    web_socket_api::WebSocket,
};

mod common;

#[tokio::test]
async fn ws_order_book() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = OrderBookParams::new("BTCUSDT").limit(10);
    let resp = client.market().order_book().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_recent_trades_list() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = RecentTradesListParams::new("BTCUSDT").limit(10);
    let resp = client.market().recent_trades_list().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_old_trade_lookup() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = OldTradeLookupParams::new("BTCUSDT").limit(10);
    let resp = client.market().old_trade_lookup().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_aggregate_trades_list() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = AggregateTradesListParams::new("BTCUSDT").limit(10);
    let resp = client
        .market()
        .aggregate_trades_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_kline_data() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = KlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = client.market().kline_data().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_ui_kline() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = UiKlinesParams::new("BTCUSDT", KlineInterval::OneHour).limit(10);
    let resp = client.market().ui_klines().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_current_average_price() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = CurrentAveragePriceParams::new("BTCUSDT");
    let resp = client
        .market()
        .current_average_price()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_ticker_24hr() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = Ticker24hrParams::new().symbol("BTCUSDT");
    let resp = client.market().ticker_24hr().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_trading_day_ticker() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = TradingDayTickerParams::new().symbol("BTCUSDT");
    let resp = client.market().trading_day_ticker().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_symbol_price_ticker() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = SymbolPriceTickerParams::new().symbol("BTCUSDT");
    let resp = client.market().symbol_price_ticker().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_symbol_order_book_ticker() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = SymbolOrderBookTickerParams::new().symbol("BTCUSDT");
    let resp = client
        .market()
        .symbol_order_book_ticker()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn ws_rolling_window_price_change() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = RollingWindowPriceChangeParams::new().symbol("BTCUSDT");
    let resp = client
        .market()
        .rolling_window_price_change()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
